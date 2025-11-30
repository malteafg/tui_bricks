use rebrickable_database_api::*;

use csv::Reader;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn get_csv_reader<P: AsRef<Path>>(path: P) -> Result<Reader<File>, std::io::Error> {
    let file = File::open(path)?;
    Ok(Reader::from_reader(file))
}

pub struct LocalDB {
    parts: HashMap<PartId, Part>,
    colors: HashMap<ColorId, ColorRecord>,
    elements: HashMap<ElementId, ElementRecord>,

    name_to_part_id: HashMap<String, PartId>,
    name_to_color_id: HashMap<String, ColorId>,
}

impl LocalDB {
    pub fn new<P: AsRef<Path>>(parts_path: P, colors_path: P, elements_path: P) -> Self {
        let mut parts = HashMap::new();
        let mut name_to_part_id = HashMap::new();
        for rec in get_csv_reader(parts_path).unwrap().deserialize() {
            let rec: PartRecord = rec.unwrap();

            if parts.contains_key(&rec.part_num) && name_to_part_id.contains_key(&rec.name) {
                panic!("Duplicate element {:?}", rec);
            }

            name_to_part_id.insert(rec.name.clone(), rec.part_num.clone());
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
            name_to_part_id,
            name_to_color_id,
        }
    }
}

impl<'a> RebrickableDB<'a> for LocalDB {
    fn part_from_id(&'a self, id: &PartId) -> Option<Cow<'a, Part>> {
        self.parts.get(id).map(Cow::Borrowed)
    }

    fn part_from_name(&'a self, name: &str) -> Option<Cow<'a, Part>> {
        let part_id = self.name_to_part_id.get(name)?;
        self.parts.get(part_id).map(Cow::Borrowed)
    }

    fn color_from_id(&'a self, id: &ColorId) -> Option<Cow<'a, ColorRecord>> {
        self.colors.get(id).map(Cow::Borrowed)
    }

    fn color_from_name(&'a self, name: &str) -> Option<Cow<'a, ColorRecord>> {
        let color_id = self.name_to_color_id.get(name)?;
        self.colors.get(color_id).map(Cow::Borrowed)
    }

    fn element_from_id(&'a self, id: &ElementId) -> Option<Cow<'a, ElementRecord>> {
        self.elements.get(id).map(Cow::Borrowed)
    }

    fn iter_part_id(&'a self) -> impl Iterator<Item = Cow<'a, PartId>> {
        self.parts.keys().map(Cow::Borrowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use std::path::PathBuf;
    use utils::PathExt;

    #[fixture]
    fn database() -> LocalDB {
        let mut parts_path = utils::crate_root!();
        parts_path.push("test-data/parts_example.csv");

        let mut colors_path = utils::crate_root!();
        colors_path.push("test-data/colors_example.csv");

        let mut elements_path = utils::crate_root!();
        elements_path.push("test-data/elements_example.csv");

        LocalDB::new(&parts_path, &colors_path, &elements_path)
    }

    #[rstest]
    fn test_parts(database: LocalDB) {
        assert_eq!(database.parts.len(), 3);

        let part = &database.parts.get(&"3021".into()).unwrap();
        assert_eq!(*part.part_record.part_num, "3021");
        assert_eq!(part.part_record.name, "Plate 2 x 3");
        assert_eq!(part.part_record.part_cat_id, "14");
        assert_eq!(part.part_record.part_material, "Plastic");
        assert_eq!(part.element_ids.len(), 3);

        let part = &database.parts.get(&"3794b".into()).unwrap();
        assert_eq!(*part.part_record.part_num, "3794b");
        assert_eq!(
            part.part_record.name,
            "Plate Special 1 x 2 with 1 Stud with Groove (Jumper)"
        );
        assert_eq!(part.part_record.part_cat_id, "9");
        assert_eq!(part.part_record.part_material, "Plastic");
        assert_eq!(part.element_ids.len(), 3);

        let part = &database.parts.get(&"4070".into()).unwrap();
        assert_eq!(*part.part_record.part_num, "4070");
        assert_eq!(part.part_record.name, "Brick Special 1 x 1 with Headlight");
        assert_eq!(part.part_record.part_cat_id, "5");
        assert_eq!(part.part_record.part_material, "Plastic");
        assert_eq!(part.element_ids.len(), 5);
    }

    #[rstest]
    fn test_colors(database: LocalDB) {
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
    fn test_elements(database: LocalDB) {
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
    fn name_to_part_id(database: LocalDB) {
        assert_eq!(database.parts.len(), database.name_to_part_id.len());
        assert_eq!(
            **database.name_to_part_id.get("Plate 2 x 3").unwrap(),
            "3021"
        );
    }

    #[test]
    fn dump() {
        let mut parts_path = PathBuf::data_dir();
        parts_path.push("parts.csv");

        let mut colors_path = PathBuf::data_dir();
        colors_path.push("colors.csv");

        let mut elements_path = PathBuf::data_dir();
        elements_path.push("elements.csv");

        let database = LocalDB::new(&parts_path, &colors_path, &elements_path);

        dbg!(&database.color_from_id(&3.into()));

        let part = database.part_from_id(&"4070".into()).unwrap();
        dbg!(&part);
        let mut colors = Vec::new();
        for id in &part.element_ids {
            let element = database.element_from_id(id).unwrap();
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
