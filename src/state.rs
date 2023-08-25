use std::io::Write;
use std::path::PathBuf;

use crossterm::{cursor, execute, queue};

use crate::command::Cmd;
use crate::data::{Database, Item, COMP_COLORS};
use crate::display;
use crate::error::{Error, Result};
use crate::input;
use crate::mode::Mode;

pub struct State {
    db: Database,
    mode: Mode,
}

impl State {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let db = Database::new(db_path)?;
        let mode = Mode::Default {
            info: "Type any of the following characters to execute the associated command"
                .to_owned(),
        };
        Ok(Self { db, mode })
    }

    pub fn accept_cmd<W: std::io::Write>(&mut self, w: &mut W) -> Result<bool> {
        self.mode.emit_mode(w)?;

        let possible_cmds = self.mode.get_possible_cmds();
        display::emit_dash(w)?;
        display::emit_line(w, "List of possible commands:")?;
        queue!(w, cursor::MoveToNextLine(1))?;
        display::emit_iter(w, possible_cmds.iter())?;

        w.flush()?;

        let cmd_char = match input::wait_for_cmdchar() {
            Ok(c) => c,
            Err(Error::Escape) => return Ok(false),
            Err(e) => return Err(e),
        };
        let Some(cmd) = possible_cmds.get(cmd_char) else {
            // self.mode = Mode::Default {
            //     info: "executing command failed".to_owned(),
            // };
            return Ok(false);
        };

        use Cmd::*;
        let new_mode = match cmd {
            Quit => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape)?;
                return Ok(true);
            }
            AddItem => self.add_item(w),
            SearchItem => self.search_item(w),
            Edit => self.edit_item(),
            QuitEdit => self.cancel_edit(w),
            SaveEdit => self.save_edit(),
            AddColorGroup => self.add_color_group(w),
            RemoveColorGroup => self.remove_color_group(w),
            EditName => self.edit_name(w),
            EditAmount => self.edit_amount(w),
            DeleteItem => self.delete_item(w),
        };

        if let Err(Error::Escape) = new_mode {
            return Ok(false);
        }

        self.mode = new_mode?;
        return Ok(false);
    }

    fn add_item<W: Write>(&mut self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        display::emit_line(w, "Adding a new item to the database")?;
        let part_id = display::input_u32(w, "Enter the part ID of the new item")?;

        if self.db.contains(part_id) {
            let item = self.db.get_item(part_id)?;
            let msg = Some(format!(
                "Item with ID {} already exists in database",
                part_id
            ));
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg,
            });
        }

        display::clear(w)?;
        display::emit_line(w, "Adding a new item to the database")?;
        let color_group = display::select_from_list(
            w,
            "Select a color group by typing its first letter\n(you can add more groups later)",
            &COMP_COLORS,
        )?;

        display::clear(w)?;
        display::emit_line(w, "Adding a new item to the database")?;
        let part_loc =
            display::input_string(w, &format!("Enter location of group {}:", color_group))?;

        let new_item = Item::new(part_id, color_group, part_loc.to_owned());
        self.db.add_item(new_item.clone())?;
        Ok(Mode::DisplayItem {
            item: new_item,
            msg: None,
        })
    }

    fn search_item<W: Write>(&self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        let searched_id = display::input_u32(w, "Enter the part ID of the new to search for.")?;

        if let Ok(item) = self.db.get_item(searched_id) {
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg: None,
            });
        }

        Ok(Mode::Default {
            info: format!("Part {} not found in database", searched_id),
        })
    }

    fn edit_item(&self) -> Result<Mode> {
        let Mode::DisplayItem { item, msg: _ } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };
        Ok(Mode::EditItem { item: item.clone() })
    }

    fn save_edit(&mut self) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };
        self.db.update_item(item.clone())?;
        Ok(Mode::DisplayItem {
            item: item.clone(),
            msg: None,
        })
    }

    fn cancel_edit<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };

        let old_item = self.db.get_item(item.get_id())?;
        if old_item == item {
            Ok(Mode::DisplayItem {
                item: item.clone(),
                msg: None,
            })
        } else {
            display::clear(w)?;
            let changes = format!(
                "Are you sure you want to quit editing and cancel these changes?\n{}",
                display::construct_item_changes(old_item, item)
            );
            if display::confirmation_prompt(w, &changes)? {
                Ok(Mode::DisplayItem {
                    item: old_item.clone(),
                    msg: None,
                })
            } else {
                Ok(Mode::EditItem { item: item.clone() })
            }
        }
    }

    fn edit_name<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };

        display::clear(w)?;
        display::emit_line(w, format!("Editing name of part: {}", item.get_id()))?;
        let new_name = display::input_string(w, "Enter new name:")?;

        let mut new_item = item.clone();
        new_item.set_name(&new_name);
        Ok(Mode::EditItem { item: new_item })
    }

    fn edit_amount<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };

        display::clear(w)?;
        display::emit_line(w, format!("Editing amount of part: {}", item.get_id()))?;
        let new_amount = display::input_u32(w, "Enter new amount:")?;

        let mut new_item = item.clone();
        new_item.set_amount(Some(new_amount));
        Ok(Mode::EditItem { item: new_item })
    }

    fn add_color_group<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };

        display::clear(w)?;
        display::emit_line(
            w,
            format!(
                "Adding a new color group to item with ID: {}",
                item.get_id()
            ),
        )?;

        let mut options = COMP_COLORS.clone().to_vec();
        options.retain(|(_, g)| {
            for (other, _) in item.get_location() {
                if other == g {
                    return false;
                }
            }
            true
        });

        let color_group = display::select_from_list(
            w,
            "Select a color group for which to add a location",
            &options,
        )?;

        display::clear(w)?;
        display::emit_line(
            w,
            format!(
                "Adding a new color group to item with ID: {}",
                item.get_id()
            ),
        )?;

        let part_loc =
            display::input_string(w, &format!("Enter location of group {}:", color_group))?;
        let mut new_item = item.clone();
        new_item.add_color_group(color_group, part_loc);
        Ok(Mode::EditItem { item: new_item })
    }

    fn remove_color_group<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };

        display::clear(w)?;
        display::emit_line(
            w,
            format!(
                "Removing a color group from item with ID: {}",
                item.get_id()
            ),
        )?;

        let mut options = COMP_COLORS.clone().to_vec();
        options.retain(|(_, g)| {
            for (other, _) in item.get_location() {
                if other == g {
                    return true;
                }
            }
            false
        });

        let color_group = display::select_from_list(w, "Select  color group to remove:", &options)?;
        let mut new_item = item.clone();
        new_item.remove_color_group(color_group);
        Ok(Mode::EditItem { item: new_item })
    }

    fn delete_item<W: Write>(&mut self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };

        display::clear(w)?;
        let changes = format!(
            "Are you absolutely sure that you want to delete the item with ID: {}?\n",
            item.get_id(),
        );
        display::emit_iter(w, item.to_string().split("\n"))?;

        if display::confirmation_prompt(w, &changes)? {
            self.db.remove_item(item.get_id())?;
            Ok(Mode::Default {
                info: format!("Item with ID: {} was deleted.", item.get_id()),
            })
        } else {
            Ok(Mode::EditItem { item: item.clone() })
        }
    }
}
