use std::fmt;

use crate::command::{Cmd, CmdList};
use crate::data::Item;

pub enum Mode {
    Default { info: String },
    DisplayItem { item: Item },
    EditItem { item: Item },
}

impl Mode {
    pub fn get_possible_cmds(&self) -> CmdList {
        use Cmd::*;
        use Mode::*;
        match self {
            Default { .. } => CmdList::new(vec![AddItem, SearchItem, Quit]),
            DisplayItem { .. } => CmdList::new(vec![AddItem, SearchItem, Quit, Edit]),
            EditItem { .. } => CmdList::new(vec![
                SaveEdit,
                CancelEdit,
                EditName,
                EditAmount,
                RemoveColorGroup,
                AddColorGroup,
            ]),
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
