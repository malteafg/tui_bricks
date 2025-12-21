use std::collections::BTreeSet;
use std::fmt;
use std::io::ErrorKind;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use term_lib::command::Command;

use crate::error::{Error, Result};
use crate::io;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, EnumIter, Ord, PartialOrd)]
pub enum ColorGroup {
    All,
    Basic,
    Earth,
    Grey,
    Road,
    Nice,
    Build,
    Translucent,
    Colorful,
    Misc,
    Other(String),
}

impl Command for ColorGroup {
    fn get_char(&self) -> char {
        use ColorGroup::*;
        match &self {
            All => 'a',
            Basic => 'b',
            Earth => 'e',
            Grey => 'g',
            Road => 'r',
            Nice => 'n',
            Build => 'u',
            Translucent => 't',
            Colorful => 'c',
            Misc => 'm',
            Other(_) => 'o',
        }
    }

    fn get_info(&self) -> &str {
        use ColorGroup::*;
        match &self {
            All => "All",
            Basic => "Basic",
            Earth => "Earth",
            Grey => "Grey",
            Road => "Road",
            Nice => "Nice",
            Build => "Build",
            Translucent => "Translucent",
            Colorful => "Colorful",
            Misc => "Misc",
            Other(name) => name,
        }
    }
}

impl fmt::Display for ColorGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_info())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Item {
    id: u32,
    alternative_ids: Vec<u32>,
    name: String,
    location: Vec<(ColorGroup, String)>,
}

impl Item {
    pub fn new(id: u32, name: String) -> Self {
        Item {
            id,
            alternative_ids: Vec::new(),
            name,
            location: Vec::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn is_group(&self) -> bool {
        self.id >= 1 << 31
    }

    pub fn get_alternative_ids(&self) -> &[u32] {
        &self.alternative_ids
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_locations(&self) -> &[(ColorGroup, String)] {
        &self.location
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_color_set(&self) -> BTreeSet<&ColorGroup> {
        self.location.iter().map(|(c, _)| c).collect()
    }

    pub fn get_other_color_set(&self) -> BTreeSet<String> {
        let mut res = BTreeSet::new();
        for (c, _) in self.location.iter() {
            if let ColorGroup::Other(s) = c {
                res.insert(s.to_string());
            }
        }
        res
    }

    pub fn add_color_group(&mut self, color_group: ColorGroup, location: String) {
        self.location.push((color_group, location))
    }

    pub fn remove_color_group(&mut self, color_group: &ColorGroup) {
        self.location.retain(|(c, _)| c != color_group);
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
        if self.get_name() != other.get_name() {
            diff.push_str(&format!(
                "Name: {} -> {}\n",
                self.get_name(),
                other.get_name()
            ));
        }
        for id in self.get_alternative_ids().iter() {
            if !other.get_alternative_ids().contains(id) {
                diff.push_str(&format!("Removed alt ID: {}\n", id));
            }
        }
        for id in other.get_alternative_ids().iter() {
            if !self.get_alternative_ids().contains(id) {
                diff.push_str(&format!("Added alt ID: {}\n", id));
            }
        }
        for loc in self.get_locations().iter() {
            if !other.get_locations().contains(loc) {
                diff.push_str(&format!("Removed location: ({}, {})\n", loc.0, loc.1));
            }
        }
        for loc in other.get_locations().iter() {
            if !self.get_locations().contains(loc) {
                diff.push_str(&format!("Added location: ({}, {})\n", loc.0, loc.1));
            }
        }
        diff
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;

        let altids = if self.alternative_ids.is_empty() {
            "None".to_string()
        } else {
            let mut res = String::new();
            for id in self.alternative_ids.iter() {
                res.push_str(&format!("{}, ", id));
            }
            res
        };

        let loc_string = if self.location.is_empty() {
            "There is currently no location for this item".to_string()
        } else {
            let mut loc_string = "Location of each color group:\n".to_owned();
            for (color_group, loc) in self.location.iter() {
                loc_string.push_str(&format!("{}: {}\n", color_group.to_string(), loc));
            }
            loc_string
        };

        if self.is_group() {
            let id = self.id & !(1 << 31);
            write!(
                f,
                "Group ID: {}\nContained part IDs: {}\n\nName: {}\n\n{}",
                id, altids, name, loc_string,
            )
        } else {
            write!(
                f,
                "Part ID: {}\nAlternative IDs: {}\n\nName: {}\n\n{}",
                self.id, altids, name, loc_string,
            )
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
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

#[derive(Serialize, Deserialize)]
pub struct Database {
    raw_data: RawDatabase,
    db_path: PathBuf,
    other_color_groups: BTreeSet<String>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        match io::read_contents_from_yaml::<_, RawDatabase>(&db_path) {
            Ok(raw_data) => Ok({
                let mut other_color_groups = BTreeSet::new();
                for item in raw_data.items.iter() {
                    for c in item.get_color_set() {
                        if let ColorGroup::Other(name) = c {
                            other_color_groups.insert(name.to_string());
                        }
                    }
                }
                Self {
                    raw_data,
                    db_path,
                    other_color_groups,
                }
            }),
            Err(term_lib::Error::IOError(io_error)) if io_error.kind() == ErrorKind::NotFound => {
                let raw_data = RawDatabase::default();
                let other_color_groups = BTreeSet::new();

                let db = Self {
                    raw_data,
                    db_path,
                    other_color_groups,
                };
                db.write()?;
                Ok(db)
            }
            Err(e) => return Err(e.into()),
        }
    }

    pub fn write(&self) -> Result<()> {
        io::write_contents_to_yaml(&self.db_path, &self.raw_data)?;
        Ok(())
    }

    pub fn add_item(&mut self, item: Item) -> Result<()> {
        if self.contains_id(item.get_id()).is_some() {
            return Err(Error::PartAlreadyExists {
                part_id: item.get_id(),
            });
        }

        for c in item.get_color_set() {
            if let ColorGroup::Other(name) = c {
                self.other_color_groups.insert(name.to_string());
            }
        }

        self.raw_data.push(item);
        self.write()?;

        Ok(())
    }

    pub fn update_item(&mut self, old_item: &Item, new_item: &Item) -> Result<()> {
        for c in new_item.get_color_set() {
            if let ColorGroup::Other(name) = c {
                self.other_color_groups.insert(name.to_string());
            }
        }

        if old_item.get_id() != new_item.get_id() {
            self.remove_item(old_item.get_id())?;
            self.add_item(new_item.clone())?;
            return Ok(());
        }

        if let Some((i, _)) = self
            .raw_data
            .iter()
            .enumerate()
            .find(|&(_, item)| item.get_id() == old_item.get_id())
        {
            self.raw_data[i] = new_item.clone();
            self.write()?;

            Ok(())
        } else {
            Err(Error::PartNotFoundId {
                part_id: old_item.get_id(),
            })
        }
    }

    /// TODO maybe do a bin or something similar
    pub fn remove_item(&mut self, id: u32) -> Result<()> {
        self.raw_data.retain(|item| item.get_id() != id);
        self.write()?;
        Ok(())
    }

    pub fn contains_id(&self, part_id: u32) -> Option<u32> {
        for item in self.raw_data.iter() {
            if item.get_id() == part_id || item.get_alternative_ids().contains(&part_id) {
                return Some(item.get_id());
            }
        }
        return None;
    }

    pub fn contains_name(&self, name: &str) -> Option<u32> {
        for item in self.raw_data.iter() {
            if item.get_name().to_lowercase() == name.to_lowercase() {
                return Some(item.get_id());
            }
        }
        return None;
    }

    pub fn get_item_by_id(&self, part_id: u32) -> Result<&Item> {
        for item in self.raw_data.iter() {
            if item.get_id() == part_id || item.get_alternative_ids().contains(&part_id) {
                return Ok(&item);
            }
        }
        Err(Error::PartNotFoundId { part_id })
    }

    pub fn get_item_by_name(&self, name: &str) -> Result<&Item> {
        for item in self.raw_data.iter() {
            if item.get_name().to_lowercase() == name.to_lowercase() {
                return Ok(&item);
            }
        }
        Err(Error::PartNotFoundName {
            name: name.to_string(),
        })
    }

    pub fn get_all_names_string(&self) -> String {
        let mut res = String::new();
        for item in self.raw_data.iter() {
            res += item.get_name();
            res.push('\n');
        }
        res
    }

    pub fn get_all_locations(&self) -> BTreeSet<&String> {
        let mut locs = BTreeSet::new();
        for item in self.raw_data.iter() {
            for (_, loc) in item.get_locations() {
                locs.insert(loc);
            }
        }
        locs
    }

    pub fn get_all_locations_string(&self) -> String {
        let locs = self.get_all_locations();
        let mut res = String::new();
        for loc in locs.into_iter() {
            res += loc;
            res += "\n";
        }
        res
    }

    pub fn get_items_at_location<'a>(
        &'a self,
        loc: &str,
    ) -> impl Iterator<Item = LocSearch<'a>> + Clone + 'a {
        let loc = loc.to_string();
        self.raw_data.iter().filter_map(move |item| {
            let mut color_groups = Vec::new();
            for (color_group, o_loc) in item.get_locations() {
                if o_loc == &loc {
                    color_groups.push(color_group);
                }
            }

            if color_groups.is_empty() {
                None
            } else {
                Some(LocSearch {
                    id: item.get_id(),
                    name: item.get_name(),
                    color_groups,
                })
            }
        })
    }

    pub fn contains_location(&self, loc: &str) -> bool {
        for item in self.raw_data.iter() {
            for (_, o_loc) in item.get_locations() {
                if o_loc == loc {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_other_color_set(&self) -> &BTreeSet<String> {
        &self.other_color_groups
    }

    pub fn get_next_group_id(&self) -> u32 {
        let mut id = 1 << 31;
        loop {
            if self.get_item_by_id(id).is_err() {
                return id;
            } else {
                id += 1;
            }
        }
    }

    pub fn get_stats(&self) -> DatabaseStats {
        let num_items = self.raw_data.len();

        let mut num_sorts = 0;
        for item in self.raw_data.iter() {
            num_sorts += item.get_locations().len();
        }

        let num_color_groups = self.get_other_color_set().len() + ColorGroup::iter().len() - 1;
        let num_locations = self.get_all_locations().len();

        DatabaseStats {
            num_items,
            num_sorts,
            num_color_groups,
            num_locations,
        }
    }
}

#[derive(Clone, Copy)]
pub struct DatabaseStats {
    num_items: usize,
    num_sorts: usize,
    num_color_groups: usize,
    num_locations: usize,
}

impl fmt::Display for DatabaseStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parts: {}\nSorted categories: {}\nColorgroups: {}\nLocations: {}",
            self.num_items, self.num_sorts, self.num_color_groups, self.num_locations
        )
    }
}

pub struct LocSearch<'a> {
    pub id: u32,
    name: &'a str,
    color_groups: Vec<&'a ColorGroup>,
}

impl fmt::Display for LocSearch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = format!("Name: {}\n   Id: {}\n   Color Groups:", self.name, self.id);
        for color_group in self.color_groups.iter() {
            res.push_str(&format!("\n       {color_group}"));
        }
        res.push_str("\n");
        write!(f, "{res}")
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
            name: "Testid".to_string(),
            location: vec![(ColorGroup::All, String::from_str("B1A3").unwrap())],
        };

        let item2 = Item {
            id: 43,
            alternative_ids: vec![12, 14],
            name: "blah blah".to_string(),
            location: vec![(ColorGroup::All, String::from_str("B1A4").unwrap())],
        };

        let test = RawDatabase {
            items: vec![item1, item2],
        };

        test
    }
}
