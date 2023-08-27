use std::fmt;

use crossterm::{cursor, queue, style::Print};

use crate::command::{Cmd, CmdList, MultiCmd};
use crate::data::Item;
use crate::display;
use crate::error::Result;

pub enum Mode {
    Default { info: String },
    DisplayItem { item: Item, msg: Option<String> },
    EditItem { item: Item },
}

impl Mode {
    pub fn get_possible_cmds(&self) -> CmdList {
        use Cmd::*;
        use Mode::*;
        match self {
            Default { .. } => CmdList::new(vec![AddItem, MCmd(MultiCmd::SearchItem), Quit]),
            DisplayItem { .. } => {
                CmdList::new(vec![AddItem, MCmd(MultiCmd::SearchItem), Quit, Edit])
            }
            EditItem { .. } => CmdList::new(vec![
                SaveEdit,
                QuitEdit,
                EditName,
                EditAmount,
                MCmd(MultiCmd::AddToItem),
                MCmd(MultiCmd::RemoveFromItem),
                DeleteItem,
            ]),
        }
    }

    pub fn emit_mode<W: std::io::Write>(&self, w: &mut W) -> Result<()> {
        display::clear(w)?;
        use Mode::*;
        match self {
            Default { info } => {
                display::default_header(w)?;
                queue!(w, Print(info), cursor::MoveToNextLine(2))?;
            }
            DisplayItem { item, msg } => {
                if let Some(msg) = msg {
                    display::header(w, msg)?;
                } else {
                    display::header(w, &format!("Viewing item with part ID {}", item.get_id()))?;
                }
                display::emit_iter(w, item.to_string().split("\n"))?;
            }
            EditItem { item } => {
                display::header(
                    w,
                    &format!("Now editing item with part ID {}", item.get_id()),
                )?;
                display::emit_iter(w, item.to_string().split("\n"))?;
            }
        }
        Ok(())
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
