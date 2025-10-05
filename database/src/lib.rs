use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{Error, Unexpected},
};

use csv::Reader;

use derive_more::{Deref, Display, From};

fn get_csv_reader<P: AsRef<Path>>(path: P) -> Result<Reader<File>, std::io::Error> {
    let file = File::open(path)?;
    Ok(Reader::from_reader(file))
}

fn bool_deserializer<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "True" | "true" => Ok(true),
        "False" | "false" => Ok(false),
        _ => Err(Error::invalid_value(
            Unexpected::Str(s),
            &"True/true or False/false",
        )),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, From, Deref, Display)]
#[serde(transparent)]
pub struct PartNum(String);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, From, Deref, Display)]
#[serde(transparent)]
pub struct ElementId(usize);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, From, Deref, Display)]
#[serde(transparent)]
pub struct ColorId(isize);

pub trait DatabaseI {
    fn part_from_num(&self, num: &PartNum) -> Option<&Part>;

    fn part_from_name(&self, name: &str) -> Option<&Part>;

    fn color_from_id(&self, id: &ColorId) -> Option<&ColorRecord>;

    fn color_from_name(&self, name: &str) -> Option<&ColorRecord>;

    fn element(&self, id: &ElementId) -> Option<&ElementRecord>;

    fn iter_part_num(&self) -> impl Iterator<Item = &PartNum>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct PartRecord {
    part_num: PartNum,
    name: String,
    part_cat_id: String,
    part_material: String,
}

#[derive(Debug, Clone)]
pub struct Part {
    part_record: PartRecord,
    element_ids: Vec<ElementId>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorRecord {
    id: ColorId,
    name: String,
    rgb: String,
    #[serde(deserialize_with = "bool_deserializer")]
    is_trans: bool,
    num_parts: usize,
    num_sets: usize,
    y1: Option<usize>,
    y2: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ElementRecord {
    element_id: ElementId,
    part_num: PartNum,
    color_id: ColorId,
    design_id: Option<usize>,
}

pub struct Database {
    parts: HashMap<PartNum, Part>,
    colors: HashMap<ColorId, ColorRecord>,
    elements: HashMap<ElementId, ElementRecord>,

    name_to_part_num: HashMap<String, PartNum>,
    name_to_color_id: HashMap<String, ColorId>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(parts_path: P, colors_path: P, elements_path: P) -> Self {
        let mut parts = HashMap::new();
        let mut name_to_part_num = HashMap::new();
        for rec in get_csv_reader(parts_path).unwrap().deserialize() {
            let rec: PartRecord = rec.unwrap();

            if parts.contains_key(&rec.part_num) && name_to_part_num.contains_key(&rec.name) {
                panic!("Duplicate element {:?}", rec);
            }

            name_to_part_num.insert(rec.name.clone(), rec.part_num.clone());
            parts.insert(
                rec.part_num.clone(),
                Part {
                    part_record: rec,
                    element_ids: Vec::new(),
                },
            );
        }

        let mut colors = HashMap::new();
        let mut name_to_color_id = HashMap::new();
        for rec in get_csv_reader(colors_path).unwrap().deserialize() {
            let rec: ColorRecord = rec.unwrap();

            if colors.contains_key(&rec.id) && name_to_color_id.contains_key(&rec.name) {
                panic!("Duplicate element {:?}", rec);
            }

            name_to_color_id.insert(rec.name.clone(), rec.id);
            colors.insert(rec.id, rec);
        }

        let mut elements = HashMap::new();
        for rec in get_csv_reader(elements_path).unwrap().deserialize() {
            let rec: ElementRecord = rec.unwrap();

            if elements.contains_key(&rec.element_id) {
                panic!("Duplicate element {:?}", rec);
            }

            parts
                .get_mut(&rec.part_num)
                .unwrap()
                .element_ids
                .push(rec.element_id);

            elements.insert(rec.element_id, rec);
        }

        Self {
            parts,
            colors,
            elements,
            name_to_part_num,
            name_to_color_id,
        }
    }
}

impl DatabaseI for Database {
    fn part_from_num(&self, num: &PartNum) -> Option<&Part> {
        self.parts.get(num)
    }

    fn part_from_name(&self, name: &str) -> Option<&Part> {
        let part_num = self.name_to_part_num.get(name)?;
        self.parts.get(part_num)
    }

    fn color_from_id(&self, id: &ColorId) -> Option<&ColorRecord> {
        self.colors.get(id)
    }

    fn color_from_name(&self, name: &str) -> Option<&ColorRecord> {
        let color_id = self.name_to_color_id.get(name)?;
        self.colors.get(color_id)
    }

    fn element(&self, id: &ElementId) -> Option<&ElementRecord> {
        self.elements.get(id)
    }

    fn iter_part_num(&self) -> impl Iterator<Item = &PartNum> {
        self.parts.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use std::path::PathBuf;

    #[fixture]
    fn database() -> Database {
        let mut parts_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        parts_path.push("test-data/parts_example.csv");

        let mut colors_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        colors_path.push("test-data/colors_example.csv");

        let mut elements_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        elements_path.push("test-data/elements_example.csv");

        Database::new(&parts_path, &colors_path, &elements_path)
    }

    #[rstest]
    fn test_parts(database: Database) {
        assert_eq!(database.parts.len(), 3);

        let part = &database.parts.get(&PartNum("3021".to_string())).unwrap();
        assert_eq!(*part.part_record.part_num, "3021");
        assert_eq!(part.part_record.name, "Plate 2 x 3");
        assert_eq!(part.part_record.part_cat_id, "14");
        assert_eq!(part.part_record.part_material, "Plastic");
        assert_eq!(part.element_ids.len(), 3);

        let part = &database.parts.get(&PartNum("3794b".to_string())).unwrap();
        assert_eq!(*part.part_record.part_num, "3794b");
        assert_eq!(
            part.part_record.name,
            "Plate Special 1 x 2 with 1 Stud with Groove (Jumper)"
        );
        assert_eq!(part.part_record.part_cat_id, "9");
        assert_eq!(part.part_record.part_material, "Plastic");
        assert_eq!(part.element_ids.len(), 3);

        let part = &database.parts.get(&PartNum("4070".to_string())).unwrap();
        assert_eq!(*part.part_record.part_num, "4070");
        assert_eq!(part.part_record.name, "Brick Special 1 x 1 with Headlight");
        assert_eq!(part.part_record.part_cat_id, "5");
        assert_eq!(part.part_record.part_material, "Plastic");
        assert_eq!(part.element_ids.len(), 5);
    }

    #[rstest]
    fn test_colors(database: Database) {
        assert_eq!(database.colors.len(), 4);

        let color = &database.colors.get(&(-1).into()).unwrap();
        assert_eq!(*color.id, -1);
        assert_eq!(color.name, "[Unknown]");
        assert_eq!(color.rgb, "0033B2");
        assert_eq!(color.is_trans, false);
        assert_eq!(color.num_parts, 20);
        assert_eq!(color.num_sets, 5);
        assert_eq!(color.y1, Some(2000));
        assert_eq!(color.y2, Some(2012));

        let color = &database.colors.get(&1.into()).unwrap();
        assert_eq!(*color.id, 1);
        assert_eq!(color.name, "Blue");
        assert_eq!(color.rgb, "0055BF");
        assert_eq!(color.is_trans, false);
        assert_eq!(color.num_parts, 193056);
        assert_eq!(color.num_sets, 46595);
        assert_eq!(color.y1, Some(1949));
        assert_eq!(color.y2, Some(2025));
    }

    #[rstest]
    fn test_elements(database: Database) {
        assert_eq!(database.elements.len(), 11);

        let element = &database.elements.get(&302123.into()).unwrap();
        assert_eq!(*element.element_id, 302123);
        assert_eq!(*element.part_num, "3021");
        assert_eq!(*element.color_id, 1);
        assert_eq!(element.design_id, Some(3021));

        let element = &database.elements.get(&407028.into()).unwrap();
        assert_eq!(*element.element_id, 407028);
        assert_eq!(*element.part_num, "4070");
        assert_eq!(*element.color_id, 2);
        assert_eq!(element.design_id, None);
    }

    #[rstest]
    fn name_to_part_num(database: Database) {
        assert_eq!(database.parts.len(), database.name_to_part_num.len());
        assert_eq!(
            **database.name_to_part_num.get("Plate 2 x 3").unwrap(),
            "3021"
        );
    }

    #[test]
    fn dump() {
        let mut parts_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        parts_path.push("../raw_data/parts.csv");

        let mut colors_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        colors_path.push("../raw_data/colors.csv");

        let mut elements_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        elements_path.push("../raw_data/elements.csv");

        let database = Database::new(&parts_path, &colors_path, &elements_path);

        dbg!(&database.color_from_id(&3.into()));

        let part = database.part_from_num(&"4070".to_string().into()).unwrap();
        dbg!(&part);
        let mut colors = Vec::new();
        for id in &part.element_ids {
            let element = database.element(id).unwrap();
            let color = database.color_from_id(&element.color_id).unwrap();
            colors.push((color.name.clone(), element.clone()));
        }

        colors.sort_by(|c1, c2| c1.0.cmp(&c2.0));
        dbg!(colors);
        // for color in &colors {
        //     dbg!(&color.0);
        // }
    }
}
