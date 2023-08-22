use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::io;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ColorGroup {
    All,
    Basic,
    Nature,
    Grey,
    Road,
    Translucent,
    OtherColorGroup(String),
}

impl fmt::Display for ColorGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            All => write!(f, "All"),
            Basic => write!(f, "Basic"),
            Nature => write!(f, "Nature"),
            Grey => write!(f, "Grey"),
            Road => write!(f, "Road"),
            Translucent => write!(f, "Translucent"),
            OtherColorGroup(name) => write!(f, "{}", name),
        }
    }
}

use ColorGroup::*;
pub const COMP_COLORS: [(char, ColorGroup); 6] = [
    ('a', All),
    ('b', Basic),
    ('n', Nature),
    ('g', Grey),
    ('r', Road),
    ('t', Translucent),
];

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Item {
    id: u32,
    alternative_ids: Vec<u32>,
    name: Option<String>,
    amount: Option<u32>,
    location: Vec<(ColorGroup, String)>,
}

impl Item {
    pub fn new(id: u32, color_group: ColorGroup, location: String) -> Self {
        Item {
            id,
            alternative_ids: Vec::new(),
            name: None,
            amount: None,
            location: vec![(color_group, location)],
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_name(&mut self, name: &str) {
        if name.is_empty() {
            self.name = None;
        } else {
            self.name = Some(name.to_string());
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut loc_string = "Location of each color group:\n".to_owned();
        for (color_group, loc) in self.location.iter() {
            loc_string.push_str(&format!("{}: {}\n", color_group.to_string(), loc));
        }

        let name = match &self.name {
            Some(name) => name.to_string(),
            None => "unspecified".to_string(),
        };

        let amount = match self.amount {
            Some(amount) => amount.to_string(),
            None => "unspecified".to_string(),
        };

        write!(
            f,
            "Part ID: {}\nName: {}\nAmount: {}\n{}",
            self.id, name, amount, loc_string,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawDatabase {
    items: Vec<Item>,
}

impl core::ops::Deref for RawDatabase {
    type Target = Vec<Item>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.items
    }
}

impl core::ops::DerefMut for RawDatabase {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.items
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Database {
    raw_data: RawDatabase,
    db_path: PathBuf,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = crate::io::get_default_database_path()?;
        let raw_data = crate::io::read_database_from_path(&db_path)?;
        Ok(Self { raw_data, db_path })
    }

    /// Creates a new state with a database from the given path.
    pub fn _from_path<P>(_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        todo!();
    }

    pub fn write(&self) -> Result<()> {
        io::write_database_to_path(&self.db_path, &self.raw_data)
    }

    pub fn add_item(&mut self, item: Item) -> Result<()> {
        if self.contains(item.get_id()) {
            return Err(Error::PartAlreadyExists {
                part_id: item.get_id(),
            });
        }

        self.raw_data.push(item);
        self.write()?;

        Ok(())
    }

    pub fn update_item(&mut self, item: Item) -> Result<()> {
        if let Some((i, _)) = self
            .raw_data
            .iter()
            .enumerate()
            .find(|&(_, old_item)| item.get_id() == old_item.get_id())
        {
            self.raw_data[i] = item;
            self.write()?;

            Ok(())
        } else {
            Err(Error::PartNotFound {
                part_id: item.get_id(),
            })
        }
    }

    pub fn contains(&self, part_id: u32) -> bool {
        for item in self.raw_data.iter() {
            if item.get_id() == part_id {
                return true;
            }
        }
        return false;
    }

    pub fn get_item(&self, part_id: u32) -> Option<&Item> {
        for item in self.raw_data.iter() {
            if item.get_id() == part_id {
                return Some(&item);
            }
        }
        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::str::FromStr;

    // Used for testing in io module
    pub fn get_test_database() -> RawDatabase {
        let item1 = Item {
            id: 44,
            alternative_ids: vec![123, 1324],
            name: Some(String::from_str("Testid").unwrap()),
            amount: None,
            location: vec![(ColorGroup::All, String::from_str("B1A3").unwrap())],
        };

        let item2 = Item {
            id: 43,
            alternative_ids: vec![12, 14],
            name: Some(String::from_str("blah blah").unwrap()),
            amount: None,
            location: vec![(ColorGroup::All, String::from_str("B1A4").unwrap())],
        };

        let test = RawDatabase {
            items: vec![item1, item2],
        };

        test
    }
}
