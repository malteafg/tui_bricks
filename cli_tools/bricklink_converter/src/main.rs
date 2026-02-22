use clap::Parser;
use glob;
use rebrickable_database_api::bricklink;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(required = true, num_args = 1..)]
    inputs: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let files = collect_xml_files(cli.inputs);

    if files.is_empty() {
        panic!();
    }

    for file in files {
        println!("Processing: {:?}", file);
        convert_to_csv(file);
    }
}

fn convert_to_csv(path: impl AsRef<Path>) {
    let path = path.as_ref();
    let xml = std::fs::read_to_string(&path).unwrap();

    if let Ok(parts) = quick_xml::de::from_str::<xml_records::Parts>(&xml) {
        dbg!(&parts.items[0]);
        return;
    };
    if let Ok(categories) = quick_xml::de::from_str::<xml_records::Categories>(&xml) {
        let mut new_path: PathBuf = path.into();
        new_path.set_extension("csv");
        let mut wtr = csv::Writer::from_path(new_path).unwrap();

        for cat in categories.items {
            wtr.serialize(bricklink::Category::from(cat)).unwrap();
        }
        return;
    };
    if let Ok(codes) = quick_xml::de::from_str::<xml_records::Codes>(&xml) {
        dbg!(&codes.items[0]);
        return;
    };
    if let Ok(colors) = quick_xml::de::from_str::<xml_records::Colors>(&xml) {
        dbg!(&colors.items[0]);
        return;
    };

    println!("could not parse xml file: {}", &path.display());
}

fn collect_xml_files(inputs: Vec<String>) -> Vec<PathBuf> {
    let mut results = Vec::new();

    for input in inputs {
        // Always run through glob — works for both literal paths and patterns
        for entry in glob::glob(&input).unwrap() {
            let path = entry.unwrap();

            if is_xml(&path) {
                results.push(path);
            } else {
                panic!("Non-XML file provided: {:?}", path);
            }
        }
    }

    results
}

fn is_xml(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("xml"))
        .unwrap_or(false)
}

// use utils::PathExt;

// use std::fs::File;
// use std::path::{Path, PathBuf};

mod xml_records {
    use rebrickable_database_api::bricklink;
    use serde::{Deserialize, Deserializer, Serialize};

    fn empty_string_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;

        match opt {
            Some(s) if s.trim().is_empty() => Ok(None),
            Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Categories {
        #[serde(rename = "ITEM")]
        pub items: Vec<Category>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Category {
        #[serde(rename = "CATEGORY")]
        category_id: usize,

        #[serde(rename = "CATEGORYNAME")]
        category_name: String,
    }

    impl From<Category> for bricklink::Category {
        fn from(cat: Category) -> Self {
            bricklink::Category {
                category_id: cat.category_id.into(),
                category_name: cat.category_name.into(),
            }
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Codes {
        #[serde(rename = "ITEM")]
        pub items: Vec<Code>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Code {
        #[serde(rename = "ITEMTYPE")]
        item_type: String,

        #[serde(rename = "ITEMID")]
        item_id: String,

        #[serde(rename = "COLOR")]
        color: String,

        #[serde(rename = "CODENAME")]
        code_name: usize,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Colors {
        #[serde(rename = "ITEM")]
        pub items: Vec<Color>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Color {
        #[serde(rename = "COLOR")]
        color_id: usize,

        #[serde(rename = "COLORNAME")]
        color_name: String,

        #[serde(rename = "COLORRGB", deserialize_with = "empty_string_as_none")]
        color_rgb: Option<String>,

        #[serde(rename = "COLORTYPE")]
        code_type: String,

        #[serde(rename = "COLORYEARFROM", deserialize_with = "empty_string_as_none")]
        color_from: Option<usize>,

        #[serde(rename = "COLORYEARTO", deserialize_with = "empty_string_as_none")]
        color_to: Option<usize>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Parts {
        #[serde(rename = "ITEM")]
        pub items: Vec<Part>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Part {
        #[serde(rename = "ITEMTYPE")]
        item_type: String,

        #[serde(rename = "ITEMID")]
        item_id: String,

        #[serde(rename = "ITEMNAME")]
        item_name: String,

        #[serde(rename = "ALTITEMIDS")]
        alt_item_ids: String,

        #[serde(rename = "CATEGORY")]
        category: String,

        #[serde(rename = "ITEMWEIGHT", deserialize_with = "empty_string_as_none")]
        item_weight: Option<f32>,

        #[serde(rename = "ITEMDIMX", deserialize_with = "empty_string_as_none")]
        item_dim_x: Option<f32>,

        #[serde(rename = "ITEMDIMY", deserialize_with = "empty_string_as_none")]
        item_dim_y: Option<f32>,

        #[serde(rename = "ITEMDIMZ", deserialize_with = "empty_string_as_none")]
        item_dim_z: Option<f32>,
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use records::*;

//     #[test]
//     fn test() {
//         let mut parts_path = PathBuf::data_dir();
//         parts_path.push("bricklink/Parts.xml");
//         let file = std::fs::File::open(parts_path).unwrap();
//         let reader = std::io::BufReader::new(file);

//         let catalog: Catalog = quick_xml::de::from_reader(reader).unwrap();
//         for i in 0..10 {
//             dbg!(&catalog.items[i]);
//         }
//     }
// }
