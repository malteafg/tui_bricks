use std::io::Write;

use strum::IntoEnumIterator;

use term_lib::cmd::CmdList;
use term_lib::display;
use term_lib::{Error, Result};

use crate::cmd::Cmds;
use crate::data::{ColorGroup, Database, Item};

pub enum Modes {
    Default { info: String },
    DisplayItem { item: Item, msg: Option<String> },
    EditItem { item: Item, msg: Option<String> },
}

impl crate::Mode for Modes {
    fn exec<W: Write>(
        &mut self,
        w: &mut W,
        state: &mut Database,
        cmd: Cmds,
    ) -> Result<Option<Modes>> {
        match self {
            Modes::Default { info: _ } => handle_default(w, state, cmd),
            Modes::DisplayItem { item, msg: _ } => {
                handle_display_item(w, item, state, cmd)
            }
            Modes::EditItem { item, msg: _ } => {
                handle_edit_item(w, item, state, cmd)
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

fn handle_default<W: Write>(
    w: &mut W,
    db: &mut Database,
    cmd: Cmds,
) -> Result<Option<Modes>> {
    match cmd {
        Cmds::AddItem => add_item(db, w),
        Cmds::Quit => return Err(Error::Quit),
        _ => panic!(),
    }
}

fn handle_display_item<W: std::io::Write>(
    w: &mut W,
    old_item: &mut Item,
    db: &mut Database,
    cmd: Cmds,
) -> Result<Option<Modes>> {
    match cmd {
        Cmds::AddItem => add_item(db, w),
        Cmds::Quit => return Err(Error::Quit),
        Cmds::Edit => todo!(),
        _ => panic!(),
    }
}

fn handle_edit_item<W: std::io::Write>(
    w: &mut W,
    old_item: &mut Item,
    db: &mut Database,
    cmd: Cmds,
) -> Result<Option<Modes>> {
    todo!()
}

fn add_item<W: Write>(db: &mut Database, w: &mut W) -> Result<Option<Modes>> {
    display::clear(w)?;
    display::emit_line(w, "Adding a new item to the database")?;
    let part_id = display::input_u32(w, "Enter the part ID of the new item")?;

    if let Some(main_id) = db.contains_id(part_id) {
        let item = db.get_item_by_id(part_id).unwrap();
        let msg = Some(format!(
                "Item with part ID {} already exists in database under item with part ID {}",
                part_id, main_id
            ));
        return Ok(Some(Modes::DisplayItem {
            item: item.clone(),
            msg,
        }));
    }

    display::clear(w)?;
    display::emit_line(w, "Adding a new item to the database")?;
    let color_group = display::select_from_list_char(
            w,
            "Select a color group by typing its first letter\n(you can add more groups later)",
            &ColorGroup::iter().collect(),
        )?;

    display::clear(w)?;
    display::emit_line(w, "Adding a new item to the database")?;
    let part_loc = display::input_string(
        w,
        &format!("Enter location of group {}:", color_group),
    )?;
    let part_loc = part_loc.to_uppercase();

    let new_item = Item::new(part_id, color_group, part_loc.to_owned());
    db.add_item(new_item.clone()).unwrap();
    Ok(Some(Modes::DisplayItem {
        item: new_item,
        msg: None,
    }))
}
