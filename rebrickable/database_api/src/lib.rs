use utils::DisplayShort;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Display,
};

utils::strong_type!(PartId, String);
utils::strong_type!(ColorId, isize, Copy);
utils::strong_type!(ElementId, usize, Copy);
utils::strong_type!(CategoryId, usize, Copy);

utils::strong_type!(PartName, String);
utils::strong_type!(ColorName, String);
utils::strong_type!(CategoryName, String);

impl PartId {
    pub fn trim_id(&self) -> Option<Self> {
        if !self.starts_with(|c: char| c.is_ascii_digit()) {
            return None;
        }

        let index = self.find(|c: char| !c.is_ascii_digit());

        let Some(index) = index else {
            return None;
        };

        Some(self[0..index].to_string().into())
    }
}

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
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Deserializer, Serialize};

    use crate::RelationshipType;

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct PartRecord {
        // Named part_num and not part_id, because this matches rebrickable CSV representation.
        pub part_num: super::PartId,
        pub name: super::PartName,
        pub part_cat_id: super::CategoryId,
        pub part_material: String,
    }

    fn bool_deserializer<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "True" | "true" => Ok(true),
            "False" | "false" => Ok(false),
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"True/true or False/false",
            )),
        }
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

    fn relationship_type_deserializer<'de, D>(deserializer: D) -> Result<RelationshipType, D::Error>
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

    #[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
    pub struct CategoryRecord {
        pub id: super::CategoryId,
        pub name: super::CategoryName,
    }
}

pub use records::{CategoryRecord, ColorRecord, ElementRecord, PartRecord, RelationshipRecord};

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
pub struct Part {
    pub part_record: PartRecord,
    pub colors: BTreeMap<ColorName, BTreeSet<ElementId>>,
    pub parent_rels: BTreeMap<PartId, BTreeSet<RelationshipType>>,
    pub child_rels: BTreeMap<PartId, BTreeSet<RelationshipType>>,
    pub category_name: CategoryName,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Part Name: {}", self.part_record.name)?;
        writeln!(f, "Id: {}", self.part_record.part_num)?;
        writeln!(
            f,
            "Category: {} ({})",
            self.category_name, self.part_record.part_cat_id
        )?;
        writeln!(f, "Material: {}", self.part_record.part_material)?;
        writeln!(f, "Child parts:")?;
        for (child_id, rel_types) in &self.child_rels {
            writeln!(f, "    {}, {:?}", child_id, rel_types)?;
        }
        writeln!(f, "Parent parts:")?;
        for (parent_id, rel_types) in &self.parent_rels {
            writeln!(f, "    {}, {:?}", parent_id, rel_types)?;
        }
        write!(f, "Color variations: {} unique colors:", self.colors.len())?;
        for (color_name, elements_ids) in &self.colors {
            writeln!(f, "")?;
            write!(f, "    {}, {:?}", color_name, elements_ids)?;
        }
        Ok(())
    }
}

impl DisplayShort for Part {
    fn fmt_short(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Part Name: {}", self.part_record.name)?;
        writeln!(f, "Id: {}", self.part_record.part_num)?;
        writeln!(
            f,
            "Category: {} ({})",
            self.category_name, self.part_record.part_cat_id
        )?;
        write!(f, "Material: {}", self.part_record.part_material)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
pub struct Color {
    pub color_record: ColorRecord,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Color Name: {}", self.color_record.name)?;
        writeln!(f, "Id: {}", self.color_record.id)?;
        writeln!(f, "RGB value: {}", self.color_record.rgb)?;
        if self.color_record.is_trans {
            writeln!(f, "Transparent: Yes")?;
        } else {
            writeln!(f, "Transparent: No")?;
        }
        writeln!(f, "Number of parts: {}", self.color_record.num_parts)?;
        writeln!(f, "Number of sets: {}", self.color_record.num_sets)?;
        write!(
            f,
            "Years active: {:?} - {:?}",
            self.color_record.y1, self.color_record.y2
        )?;
        Ok(())
    }
}

impl DisplayShort for Color {
    fn fmt_short(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Color Name: {}", self.color_record.name)?;
        writeln!(f, "Id: {}", self.color_record.id)?;
        write!(f, "RGB value: {}", self.color_record.rgb)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
pub struct Element {
    pub element_record: ElementRecord,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Element Id: {}", self.element_record.element_id)?;
        writeln!(f, "Part Id: {}", self.element_record.part_num)?;
        writeln!(f, "Color Id: {}", self.element_record.color_id)?;
        write!(f, "Design Id: {:?}", self.element_record.design_id)?;
        Ok(())
    }
}

impl DisplayShort for Element {
    fn fmt_short(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode)]
pub struct Category {
    pub category_record: CategoryRecord,
    pub parts: HashSet<PartId>,
}

pub trait RebrickableDB {
    fn part_from_id(&self, id: &PartId) -> Option<Cow<Part>>;

    fn part_from_name(&self, name: &PartName) -> Option<Cow<Part>>;

    fn color_from_id(&self, id: &ColorId) -> Option<Cow<Color>>;

    fn color_from_name(&self, name: &ColorName) -> Option<Cow<Color>>;

    fn element_from_id(&self, id: &ElementId) -> Option<Cow<Element>>;

    fn iter_part_id(&self) -> impl Iterator<Item = Cow<PartId>>;

    fn iter_part_name(&self) -> impl Iterator<Item = Cow<PartName>>;

    fn iter_color_id(&self) -> impl Iterator<Item = Cow<ColorId>>;

    fn iter_color_name(&self) -> impl Iterator<Item = Cow<ColorName>>;

    fn iter_element_id(&self) -> impl Iterator<Item = Cow<ElementId>>;
}
