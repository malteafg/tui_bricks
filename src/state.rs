use std::path::{Path, PathBuf};

use crate::command::CmdList;
use crate::data::{Database, Item};
use crate::error::{Error, Result};
use crate::io;

pub struct State {
    db: Database,
    db_path: PathBuf,
    mode: Mode,
}

pub enum Mode {
    Default { info: String },
    DisplayItem { item: Item },
}

impl Mode {
    pub fn get_possible_cmds(&self) -> CmdList {
        use crate::command::*;
        use Mode::*;
        match self {
            Default { .. } => CmdList::new(vec![ADD_ITEM, SEARCH_ITEM]),
            DisplayItem { .. } => CmdList::new(vec![ADD_ITEM, SEARCH_ITEM]),
        }
    }
}

impl State {
    pub fn new() -> Result<Self> {
        let db_path = io::get_default_database_path()?;
        let db = io::read_database_from_path(&db_path)?;
        let mode = Mode::Default {
            info: "Type any of the following characters to execute the associated command"
                .to_owned(),
        };
        Ok(Self { db, db_path, mode })
    }

    /// Creates a new state with a database from the given path.
    pub fn _from_path<P>(_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        todo!();
    }

    pub fn write(&self) -> Result<()> {
        io::write_database_to_path(&self.db_path, &self.db)
    }

    pub fn add_item(&mut self, item: Item) -> Result<()> {
        if self.contains(item.get_id()) {
            return Err(Error::PartExists {
                part_id: item.get_id(),
            });
        }

        self.db.push(item);
        self.write()?;

        Ok(())
    }

    pub fn contains(&self, part_id: u32) -> bool {
        for item in self.db.iter() {
            if item.get_id() == part_id {
                return true;
            }
        }
        return false;
    }

    pub fn get_item(&self, part_id: u32) -> Option<&Item> {
        for item in self.db.iter() {
            if item.get_id() == part_id {
                return Some(&item);
            }
        }

        None
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
