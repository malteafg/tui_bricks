use std::fmt;

use crate::command::CmdList;
use crate::data::Item;

pub enum Mode {
    Default { info: String },
    DisplayItem { item: Item },
    EditItem { item: Item },
}

impl Mode {
    pub fn get_possible_cmds(&self) -> CmdList {
        use crate::command::*;
        use Mode::*;
        match self {
            Default { .. } => CmdList::new(vec![&ADD_ITEM, &SEARCH_ITEM, &QUIT]),
            DisplayItem { .. } => CmdList::new(vec![&ADD_ITEM, &SEARCH_ITEM, &QUIT, &EDIT]),
            EditItem { .. } => CmdList::new(vec![&SAVE_EDIT, &CANCEL_EDIT]),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Mode::*;
        match self {
            Default { .. } => write!(f, "Default"),
            DisplayItem { .. } => write!(f, "Display Item"),
            EditItem { .. } => write!(f, "Edit Item"),
        }
    }
}
