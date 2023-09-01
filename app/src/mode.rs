use term_lib::cmd::CmdList;
use term_lib::display;
use term_lib::{Error, Result};

use crate::cmd::Cmds;
use crate::data::{Database, Item};

pub enum Modes {
    Default { info: String },
    DisplayItem { item: Item, msg: Option<String> },
    EditItem { item: Item, msg: Option<String> },
}

impl crate::Mode for Modes {
    fn exec(
        &mut self,
        state: &mut Database,
        cmd: Cmds,
    ) -> Result<Option<Modes>> {
        match self {
            Modes::Default { info: _ } => handle_default(state, cmd),
            Modes::DisplayItem { item, msg: _ } => {
                handle_display_item(item, state, cmd)
            }
            Modes::EditItem { item, msg: _ } => {
                handle_edit_item(item, state, cmd)
            }
        }
    }

    fn emit_mode<W: std::io::Write>(&self, w: &mut W) -> term_lib::Result<()> {
        display::clear(w)?;
        match self {
            Modes::Default { info } => {
                display::header(w, "Welcome to TUI bricks")?;
                display::emit_iter(w, info.split("\n"))?;
            }
            Modes::DisplayItem { item, msg } => {
                if let Some(msg) = msg {
                    display::header(w, msg)?;
                } else {
                    display::header(
                        w,
                        &format!("Viewing item with part ID {}", item.get_id()),
                    )?;
                }
                display::emit_iter(w, item.to_string().split("\n"))?;
            }
            Modes::EditItem { item, msg } => {
                if let Some(msg) = msg {
                    display::header(w, msg)?;
                } else {
                    display::header(
                        w,
                        &format!(
                            "Now editing item with part ID {}",
                            item.get_id()
                        ),
                    )?;
                }
                display::emit_iter(w, item.to_string().split("\n"))?;
            }
        }
        w.flush()?;
        Ok(())
    }

    fn get_possible_cmds(&self) -> term_lib::cmd::CmdList<Cmds> {
        use Cmds::*;
        match self {
            Modes::Default { .. } => {
                CmdList::new(vec![
                    AddItem, // MCmd(MultiCmd::SearchItem),
                    Quit,
                ])
            }
            Modes::DisplayItem { .. } => CmdList::new(vec![
                AddItem, // MCmd(MultiCmd::SearchItem),
                Quit, Edit,
            ]),
            Modes::EditItem { .. } => CmdList::new(vec![
                SaveEdit, QuitEdit, EditName, EditAmount,
                // MCmd(MultiCmd::AddToItem),
                // MCmd(MultiCmd::RemoveFromItem),
                DeleteItem,
            ]),
        }
    }
}

fn handle_default(db: &mut Database, cmd: Cmds) -> Result<Option<Modes>> {
    match cmd {
        Cmds::AddItem => todo!(),
        Cmds::Quit => return Err(Error::Quit),
        _ => panic!(),
    }
}

fn handle_display_item(
    old_item: &mut Item,
    db: &mut Database,
    cmd: Cmds,
) -> Result<Option<Modes>> {
    todo!()
}

fn handle_edit_item(
    old_item: &mut Item,
    db: &mut Database,
    cmd: Cmds,
) -> Result<Option<Modes>> {
    todo!()
}
