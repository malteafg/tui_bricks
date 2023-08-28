use std::collections::BTreeSet;
use std::fmt;
use std::io::ErrorKind;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::command::CmdChar;
use crate::error::{Error, Result};
use crate::io;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    EnumIter,
    Ord,
    PartialOrd,
)]
pub enum ColorGroup {
    All,
    Basic,
    Earth,
    Grey,
    Road,
    Nice,
    Translucent,
    Colorful,
}

impl CmdChar for ColorGroup {
    fn get_char(&self) -> char {
        use ColorGroup::*;
        match self {
            All => 'a',
            Basic => 'b',
            Earth => 'e',
            Grey => 'g',
            Road => 'r',
            Nice => 'n',
            Translucent => 't',
            Colorful => 'c',
        }
    }
}

impl fmt::Display for ColorGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ColorGroup::*;
        match self {
            All => write!(f, "All"),
            Basic => write!(f, "Basic"),
            Earth => write!(f, "Earth"),
            Grey => write!(f, "Grey"),
            Road => write!(f, "Road"),
            Nice => write!(f, "Nice"),
            Translucent => write!(f, "Translucent"),
            Colorful => write!(f, "Colorful"),
        }
    }
}

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

    pub fn get_alternative_ids(&self) -> &[u32] {
        &self.alternative_ids
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn get_amount(&self) -> Option<u32> {
        self.amount
    }

    pub fn get_location(&self) -> &[(ColorGroup, String)] {
        &self.location
    }

    pub fn set_name(&mut self, name: &str) {
        if name.is_empty() {
            self.name = None;
        } else {
            self.name = Some(name.to_string());
        }
    }

    pub fn set_amount(&mut self, amount: Option<u32>) {
        self.amount = amount;
    }

    pub fn get_color_set(&self) -> BTreeSet<ColorGroup> {
        self.location.iter().map(|(c, _)| *c).collect()
    }

    pub fn add_color_group(
        &mut self,
        color_group: ColorGroup,
        location: String,
    ) {
        self.location.push((color_group, location))
    }

    pub fn remove_color_group(&mut self, color_group: ColorGroup) {
        self.location.retain(|(c, _)| *c != color_group);
    }

    pub fn add_alt_id(&mut self, id: u32) {
        self.alternative_ids.push(id);
    }

    pub fn remove_alt_id(&mut self, id: u32) {
        self.alternative_ids.retain(|old_id| id != *old_id);
    }

    pub fn diff(&self, other: &Item) -> String {
        let mut diff = String::new();
        if self.get_id() != other.get_id() {
            diff.push_str(&format!(
                "Part ID: {} -> {}\n",
                self.get_id(),
                other.get_id()
            ));
        }
        if self.get_alternative_ids() != other.get_alternative_ids() {
            diff.push_str(&format!(
                "Alternative IDs: {:#?} -> {:#?}\n",
                self.get_alternative_ids(),
                other.get_alternative_ids()
            ));
        }
        if self.get_name() != other.get_name() {
            diff.push_str(&format!(
                "Name: {:#?} -> {:#?}\n",
                self.get_name(),
                other.get_name()
            ));
        }
        if self.get_amount() != other.get_amount() {
            diff.push_str(&format!(
                "Amount: {:#?} -> {:#?}\n",
                self.get_amount(),
                other.get_amount()
            ));
        }
        if self.get_location() != other.get_location() {
            diff.push_str(&format!(
                "Locations: {:#?} -> {:#?}\n",
                self.get_location(),
                other.get_location()
            ));
        }
        diff
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self.name {
            Some(name) => name.to_string(),
            None => "unspecified".to_string(),
        };

        let altids = if self.alternative_ids.is_empty() {
            "None".to_string()
        } else {
            let mut res = String::new();
            for id in self.alternative_ids.iter() {
                res.push_str(&format!("{}, ", id));
            }
            res
        };

        let amount = match self.amount {
            Some(amount) => amount.to_string(),
            None => "unspecified".to_string(),
        };

        let mut loc_string = "Location of each color group:\n".to_owned();
        for (color_group, loc) in self.location.iter() {
            loc_string.push_str(&format!(
                "{}: {}\n",
                color_group.to_string(),
                loc
            ));
        }

        write!(
            f,
            "Part ID: {}\nAlternative IDs: {}\n\nName: {}\nAmount: {}\n\n{}",
            self.id, altids, name, amount, loc_string,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
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
    pub fn new(db_path: PathBuf) -> Result<Self> {
        match io::read_contents_from_path(&db_path) {
            Ok(raw_data) => Ok(Self { raw_data, db_path }),
            Err(Error::IOError(io_error))
                if io_error.kind() == ErrorKind::NotFound =>
            {
                let raw_data = RawDatabase::default();
                let db = Self { raw_data, db_path };
                db.write()?;
                Ok(db)
            }
            Err(e) => return Err(e),
        }
    }

    pub fn write(&self) -> Result<()> {
        io::write_contents_to_path(&self.db_path, &self.raw_data)
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
            Err(Error::PartNotFoundId {
                part_id: item.get_id(),
            })
        }
    }

    /// TODO maybe do a bin or something similar
    pub fn remove_item(&mut self, id: u32) -> Result<()> {
        self.raw_data.retain(|item| item.get_id() != id);
        self.write()?;
        Ok(())
    }

    pub fn contains(&self, part_id: u32) -> bool {
        for item in self.raw_data.iter() {
            if item.get_id() == part_id {
                return true;
            }
        }
        return false;
    }

    pub fn get_item_by_id(&self, part_id: u32) -> Result<&Item> {
        for item in self.raw_data.iter() {
            if item.get_id() == part_id {
                return Ok(&item);
            }
        }
        Err(Error::PartNotFoundId { part_id })
    }

    pub fn get_item_by_name(&self, name: &str) -> Result<&Item> {
        for item in self.raw_data.iter() {
            if let Some(n) = item.get_name() {
                if n == name {
                    return Ok(&item);
                }
            }
        }
        Err(Error::PartNotFoundName {
            name: name.to_string(),
        })
    }

    pub fn get_all_names(&self) -> String {
        let mut res = String::new();
        for d in self.raw_data.iter() {
            if let Some(n) = d.get_name() {
                res.push_str(n);
                res.push('\n');
            }
        }
        res
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
            location: vec![(
                ColorGroup::All,
                String::from_str("B1A3").unwrap(),
            )],
        };

        let item2 = Item {
            id: 43,
            alternative_ids: vec![12, 14],
            name: Some(String::from_str("blah blah").unwrap()),
            amount: None,
            location: vec![(
                ColorGroup::All,
                String::from_str("B1A4").unwrap(),
            )],
        };

        let test = RawDatabase {
            items: vec![item1, item2],
        };

        test
    }
}
