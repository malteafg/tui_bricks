use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(EnumString, Display, Serialize, Deserialize, Debug, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum ColorGroup {
    All,
    Basic,
    Nature,
    Grey,
    Road,
    Translucent,
    OtherColorGroup(i32),
}

use ColorGroup::*;
pub const COMP_COLORS: [ColorGroup; 6] = [All, Basic, Nature, Grey, Road, Translucent];

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    id: u32,
    alternative_ids: Vec<u32>,
    name: String,
    location: Vec<(ColorGroup, String)>,
}

impl Item {
    pub fn new(id: u32, color_group: ColorGroup, location: String) -> Self {
        Item {
            id,
            alternative_ids: Vec::new(),
            name: String::new(),
            location: vec![(color_group, location)],
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut loc_string = "Location of each color group:\n".to_owned();
        for (color_group, loc) in self.location.iter() {
            loc_string.push_str(&format!("{}: {}\n", color_group.to_string(), loc));
        }

        write!(
            f,
            "Part found: {}\n\nPart ID: {}\n\n{}",
            self.name.clone(),
            self.id,
            loc_string,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Database {
    items: Vec<Item>,
}

impl core::ops::Deref for Database {
    type Target = Vec<Item>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.items
    }
}

impl core::ops::DerefMut for Database {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.items
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::str::FromStr;

    // Used for testing in io module
    pub fn get_test_database() -> Database {
        let item1 = Item {
            id: 44,
            alternative_ids: vec![123, 1324],
            name: String::from_str("Testid").unwrap(),
            location: vec![(ColorGroup::All, String::from_str("B1A3").unwrap())],
        };

        let item2 = Item {
            id: 43,
            alternative_ids: vec![12, 14],
            name: String::from_str("blah blah").unwrap(),
            location: vec![(ColorGroup::All, String::from_str("B1A4").unwrap())],
        };

        let test = Database {
            items: vec![item1, item2],
        };

        test
    }
}
