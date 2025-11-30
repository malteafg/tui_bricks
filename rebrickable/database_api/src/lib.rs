use utils::strong_type::Strong;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use std::borrow::Cow;

utils::strong_type!(PartId, String);
utils::strong_type!(ElementId, usize);
utils::strong_type!(ColorId, isize);

/// Records match the rebrickable CSV representation
mod records {
    use utils::serde_ext::bool_deserializer;

    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct PartRecord {
        pub part_num: super::PartId,
        pub name: String,
        pub part_cat_id: String,
        pub part_material: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct ColorRecord {
        pub id: super::ColorId,
        pub name: String,
        pub rgb: String,
        #[serde(deserialize_with = "bool_deserializer")]
        pub is_trans: bool,
        pub num_parts: usize,
        pub num_sets: usize,
        pub y1: Option<usize>,
        pub y2: Option<usize>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct ElementRecord {
        pub element_id: super::ElementId,
        pub part_num: super::PartId,
        pub color_id: super::ColorId,
        pub design_id: Option<usize>,
    }
}

pub use records::{ColorRecord, ElementRecord, PartRecord};

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
pub struct Part {
    pub part_record: PartRecord,
    pub element_ids: Vec<ElementId>,
}

pub trait RebrickableDB<'a> {
    fn part_from_id(&'a self, id: &PartId) -> Option<Cow<'a, Part>>;

    fn part_from_name(&'a self, name: &str) -> Option<Cow<'a, Part>>;

    fn color_from_id(&'a self, id: &ColorId) -> Option<Cow<'a, ColorRecord>>;

    fn color_from_name(&'a self, name: &str) -> Option<Cow<'a, ColorRecord>>;

    fn element_from_id(&'a self, id: &ElementId) -> Option<Cow<'a, ElementRecord>>;

    fn iter_part_id(&'a self) -> impl Iterator<Item = Cow<'a, PartId>>;
}
