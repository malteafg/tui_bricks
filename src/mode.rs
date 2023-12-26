use std::fmt;

use term_lib::command::CmdList;
use term_lib::display;

use crate::cmd::{Cmd, MultiCmd};
use crate::data::{DatabaseStats, Item};
use crate::error::Result;

pub enum Mode {
    Default { info: String },
    DisplayItem { item: Item, msg: Option<String> },
    EditItem { item: Item, msg: Option<String> },
    ViewStatistics { stats: DatabaseStats },
}

impl Mode {
    pub fn get_possible_cmds(&self) -> CmdList<Cmd> {
        use Cmd::*;
        use Mode::*;
        match self {
            Default { .. } => {
                CmdList::new(vec![AddItem, MCmd(MultiCmd::SearchItem), Quit, ViewStats])
            }
            DisplayItem { .. } => CmdList::new(vec![
                AddItem,
                MCmd(MultiCmd::SearchItem),
                Quit,
                Edit,
                ViewStats,
            ]),
            EditItem { .. } => CmdList::new(vec![
                SaveEdit,
                QuitEdit,
                EditName,
                MCmd(MultiCmd::AddToItem),
                MCmd(MultiCmd::RemoveFromItem),
                DeleteItem,
            ]),
            ViewStatistics { .. } => CmdList::new(vec![QuitStats]),
        }
    }

    pub fn emit_mode<W: std::io::Write>(&self, w: &mut W) -> Result<()> {
        display::clear(w)?;
        use Mode::*;
        match self {
            Default { info } => {
                display::header(w, "Welcome to TUI bricks")?;
                display::iter(w, info.split("\n"))?;
            }
            DisplayItem { item, msg } => {
                if let Some(msg) = msg {
                    display::header(w, msg)?;
                } else {
                    display::header(w, &format!("Viewing item with part ID {}", item.get_id()))?;
                }
                display::iter(w, item.to_string().split("\n"))?;
            }
            EditItem { item, msg } => {
                if let Some(msg) = msg {
                    display::header(w, msg)?;
                } else {
                    display::header(
                        w,
                        &format!("Now editing item with part ID {}", item.get_id()),
                    )?;
                }
                display::iter(w, item.to_string().split("\n"))?;
            }
            ViewStatistics { stats } => {
                display::iter(w, stats.to_string().split("\n"))?;
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
            ViewStatistics { .. } => write!(f, "View Statistics"),
        }
    }
}
