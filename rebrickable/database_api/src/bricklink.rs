use utils::PathExt;

use std::fs::File;
use std::path::{Path, PathBuf};

mod records {
    use serde::{Deserialize, Deserializer, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Catalog {
        #[serde(rename = "ITEM")]
        pub items: Vec<Item>,
    }

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
    pub struct Item {
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

#[cfg(test)]
mod tests {

    use super::*;
    use records::*;

    #[test]
    fn test() {
        let mut parts_path = PathBuf::data_dir();
        parts_path.push("bricklink/Parts.xml");
        let file = std::fs::File::open(parts_path).unwrap();
        let reader = std::io::BufReader::new(file);

        let catalog: Catalog = quick_xml::de::from_reader(reader).unwrap();
        for i in 0..10 {
            dbg!(&catalog.items[i]);
        }
    }
}
