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

#[derive(
    Debug, Copy, Clone, Deserialize, Serialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum RelationshipType {
    /// P
    Print,
    /// R
    Pair,
    /// B
    SubPart,
    /// M
    Mold,
    /// T
    Pattern,
    /// A
    Alternate,
}

/// Records match the rebrickable CSV representation
mod records {
    use utils::serde_ext::bool_deserializer;

    use bincode::{Decode, Encode};
    use serde::{Deserialize, Deserializer, Serialize};

    use crate::RelationshipType;

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

    pub fn relationship_type_deserializer<'de, D>(
        deserializer: D,
    ) -> Result<RelationshipType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "P" => Ok(RelationshipType::Print),
            "R" => Ok(RelationshipType::Pair),
            "B" => Ok(RelationshipType::SubPart),
            "M" => Ok(RelationshipType::Mold),
            "T" => Ok(RelationshipType::Pattern),
            "A" => Ok(RelationshipType::Alternate),
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"Should be P, R, B, M, T, or A",
            )),
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct RelationshipRecord {
        #[serde(deserialize_with = "relationship_type_deserializer")]
        pub rel_type: super::RelationshipType,
        pub child_part_num: super::PartId,
        pub parent_part_num: super::PartId,
    }
}

pub use records::{ColorRecord, ElementRecord, PartRecord, RelationshipRecord};

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
pub struct Part {
    pub part_record: PartRecord,
    pub colors: BTreeMap<ColorName, BTreeSet<ElementId>>,
    pub parent_rels: BTreeMap<PartId, BTreeSet<RelationshipType>>,
    pub child_rels: BTreeMap<PartId, BTreeSet<RelationshipType>>,
}

pub trait RebrickableDB {
    fn part_from_id(&self, id: &PartId) -> Option<Cow<Part>>;

    fn part_from_name(&self, name: &PartName) -> Option<Cow<Part>>;

    fn color_from_id(&self, id: &ColorId) -> Option<Cow<ColorRecord>>;

    fn color_from_name(&self, name: &ColorName) -> Option<Cow<ColorRecord>>;

    fn element_from_id(&self, id: &ElementId) -> Option<Cow<ElementRecord>>;

    fn iter_part_id(&self) -> impl Iterator<Item = Cow<PartId>>;
}
