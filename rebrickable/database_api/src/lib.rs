use utils::strong_type::Strong;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

utils::strong_type!(PartId, String);
utils::strong_type!(ElementId, usize);
utils::strong_type!(ColorId, isize);

utils::strong_type!(PartName, String);
utils::strong_type!(ColorName, String);

/// Records match the rebrickable CSV representation
mod records {
    use utils::serde_ext::bool_deserializer;

    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct PartRecord {
        // Named part_num and not part_id, because this matches rebrickable CSV representation.
        pub part_num: super::PartId,
        pub name: super::PartName,
        pub part_cat_id: String,
        pub part_material: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct ColorRecord {
        pub id: super::ColorId,
        pub name: super::ColorName,
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
    pub colors: BTreeMap<ColorName, BTreeSet<ElementId>>,
}

pub trait RebrickableDB {
    fn part_from_id(&self, id: &PartId) -> Option<Cow<Part>>;

    fn part_from_name(&self, name: &PartName) -> Option<Cow<Part>>;

    fn color_from_id(&self, id: &ColorId) -> Option<Cow<ColorRecord>>;

    fn color_from_name(&self, name: &ColorName) -> Option<Cow<ColorRecord>>;

    fn element_from_id(&self, id: &ElementId) -> Option<Cow<ElementRecord>>;

    fn iter_part_id(&self) -> impl Iterator<Item = Cow<PartId>>;
}
