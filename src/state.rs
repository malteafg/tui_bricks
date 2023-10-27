use std::collections::BTreeSet;
use std::io::Write;
use std::path::PathBuf;

use strum::IntoEnumIterator;

use term_lib::command::Command;
use term_lib::{display, input, prompt};

use crate::cmd::{Cmd, MultiCmd};
use crate::data::{ColorGroup, Database, Item};
use crate::error::{Error, Result};
use crate::mode::Mode;

macro_rules! bail {
    ( $self:expr, $c:ident ) => {
        return Err(crate::error::Error::CmdModeMismatch {
            cmd: crate::cmd::Cmd::$c.display_as_cmd(),
            mode: $self.mode.to_string(),
        })
    };
}

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

    pub fn wait_for_cmd<W: std::io::Write>(&mut self, w: &mut W) -> Result<()> {
        self.mode.emit_mode(w)?;

        let possible_cmds = self.mode.get_possible_cmds();
        display::dash(w)?;
        display::line(w, "List of possible commands:")?;
        display::newline(w, 1)?;
        display::possible_cmds(w, possible_cmds.iter())?;

        w.flush()?;

        let cmd_char = match input::wait_for_cmdchar() {
            Ok(c) => c,
            Err(term_lib::Error::Escape) => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        let Some(cmd) = possible_cmds.get(cmd_char) else {
            return Ok(());
        };

        let new_mode = self.execute_cmd(w, *cmd);

        if let Err(Error::TermError(term_lib::Error::Escape)) = new_mode {
            return Ok(());
        }

        self.mode = new_mode?;
        return Ok(());
    }

    fn handle_multi_cmd<W: std::io::Write>(
        &mut self,
        w: &mut W,
        m_cmd: MultiCmd,
    ) -> Result<Mode> {
        display::clear(w)?;
        display::line(w, m_cmd.get_header())?;

        let possible_cmds = m_cmd.get_possible_cmds();
        display::dash(w)?;
        display::newline(w, 1)?;
        display::possible_cmds(w, possible_cmds.iter())?;

        w.flush()?;

        let cmd_char = input::wait_for_cmdchar()?;
        let Some(cmd) = possible_cmds.get(cmd_char) else {
            return self.handle_multi_cmd(w, m_cmd);
        };

        self.execute_cmd(w, *cmd)
    }

    fn execute_cmd<W: std::io::Write>(
        &mut self,
        w: &mut W,
        cmd: Cmd,
    ) -> Result<Mode> {
        use Cmd::*;
        match cmd {
            Quit => {
                return Err(Error::TermError(term_lib::Error::Quit));
            }
            AddItem => self.add_item(w),
            Edit => self.edit_item(),
            QuitEdit => self.quit_edit(w),
            SaveEdit => self.save_edit(),
            EditName => self.edit_name(w),
            EditAmount => self.edit_amount(w),
            DeleteItem => self.delete_item(w),

            MCmd(m_cmd) => self.handle_multi_cmd(w, m_cmd),

            AddColorGroup => self.add_color_group(w),
            AddAltId => self.add_alt_id(w),

            RemoveColorGroup => self.remove_color_group(w),
            RemoveAltId => self.remove_alt_id(w),

            SearchPartID => self.search_by_id(w),
            SearchName => self.search_by_name(),
            SearchLocation => self.search_by_location(),
        }
    }

    fn add_item<W: Write>(&mut self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        display::line(w, "Adding a new item to the database")?;
        let part_id =
            prompt::input_u32(w, "Enter the part ID of the new item")?;

        if let Some(main_id) = self.db.contains_id(part_id) {
            let item = self.db.get_item_by_id(part_id)?;
            let msg = Some(format!(
                "Item with part ID {} already exists in database under item with part ID {}",
                part_id, main_id
            ));
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg,
            });
        }

        display::clear(w)?;
        display::line(w, "Adding a new item to the database")?;
        let mut color_group = prompt::select_cmd(
            w,
            "Select a color group by typing its first letter\n(you can add more groups later)",
            &ColorGroup::iter().collect(),
        )?;

        if let ColorGroup::Other(_) = color_group {
            display::clear(w)?;
            let color_name = prompt::input_string(
                w,
                &format!("Enter the name of the color:"),
            )?;
            color_group = ColorGroup::Other(color_name);
        }

        display::clear(w)?;
        display::line(w, "Adding a new item to the database")?;
        let part_loc = prompt::input_string(
            w,
            &format!("Enter location of group {}:", color_group),
        )?;
        let part_loc = part_loc.to_uppercase();

        let new_item = Item::new(part_id, color_group, part_loc.to_owned());
        self.db.add_item(new_item.clone())?;
        Ok(Mode::DisplayItem {
            item: new_item,
            msg: None,
        })
    }

    fn search_by_id<W: Write>(&self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        let searched_id = prompt::input_u32(
            w,
            "Enter the part ID of the new to search for.",
        )?;

        if let Ok(item) = self.db.get_item_by_id(searched_id) {
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg: None,
            });
        }

        Ok(Mode::Default {
            info: format!("Part {} not found in database", searched_id),
        })
    }

    fn search_by_name(&self) -> Result<Mode> {
        let opts = self.db.get_all_names();

        let searched_name = prompt::fzf_search(&opts)?;

        if let Ok(item) = self.db.get_item_by_name(&searched_name) {
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg: None,
            });
        }

        Ok(Mode::Default {
            info: format!("Part {} not found in database", searched_name),
        })
    }

    fn search_by_location(&self) -> Result<Mode> {
        let opts = self.db.get_all_locations();
        let searched_loc = prompt::fzf_search(&opts)?;
        let locations = self.db.get_items_at_location(&searched_loc);

        if locations.is_empty() {
            return Ok(Mode::Default {
                info: format!("{} does not contain anything", searched_loc),
            });
        }

        let mut info =
            format!("{} contains the following items:\n\n", searched_loc);
        for (id, color_group) in locations.iter() {
            info.push_str(&format!(
                "Part ID: {}, color group: {}",
                id, color_group
            ));
            info.push('\n');
        }

        Ok(Mode::Default { info })
    }

    fn edit_item(&self) -> Result<Mode> {
        let Mode::DisplayItem { item, msg: _ } = &self.mode else {
            bail!(self, Edit);
        };
        Ok(Mode::EditItem {
            item: item.clone(),
            msg: None,
        })
    }

    fn save_edit(&mut self) -> Result<Mode> {
        let Mode::EditItem { item, msg: _ } = &self.mode else {
            bail!(self, SaveEdit);
        };
        self.db.update_item(item.clone())?;
        Ok(Mode::DisplayItem {
            item: item.clone(),
            msg: None,
        })
    }

    fn quit_edit<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: _ } = &self.mode else {
            bail!(self, QuitEdit);
        };

        let old_item = self.db.get_item_by_id(item.get_id())?;
        if old_item == item {
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg: None,
            });
        }

        display::clear(w)?;
        let changes = format!(
                "Are you sure you want to quit editing and cancel these changes?\n{}",
                old_item.diff(item)
            );

        if prompt::confirmation(w, &changes)? {
            Ok(Mode::DisplayItem {
                item: old_item.clone(),
                msg: None,
            })
        } else {
            Ok(Mode::EditItem {
                item: item.clone(),
                msg: None,
            })
        }
    }

    fn edit_name<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: None } = &self.mode else {
            bail!(self, EditName);
        };

        display::clear(w)?;
        display::line(w, format!("Editing name of part: {}", item.get_id()))?;
        let new_name = prompt::input_string(w, "Enter new name:")?;

        if let Some(existing_id) = self.db.contains_name(&new_name) {
            return Ok(Mode::EditItem {
                item: item.clone(),
                msg: Some(format!(
                    "The item with part ID {} already has the name {}",
                    existing_id, new_name,
                )),
            });
        }

        let mut new_item = item.clone();
        new_item.set_name(&new_name);
        Ok(Mode::EditItem {
            item: new_item,
            msg: None,
        })
    }

    fn edit_amount<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: None } = &self.mode else {
            bail!(self, EditAmount);
        };

        display::clear(w)?;
        display::line(w, format!("Editing amount of part: {}", item.get_id()))?;
        let new_amount = prompt::input_u32(w, "Enter new amount:")?;

        let mut new_item = item.clone();
        new_item.set_amount(Some(new_amount));
        Ok(Mode::EditItem {
            item: new_item,
            msg: None,
        })
    }

    fn add_color_group<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: _ } = &self.mode else {
            bail!(self, AddColorGroup)
        };

        display::clear(w)?;
        display::line(
            w,
            format!(
                "Adding a new color group to item with ID: {}",
                item.get_id()
            ),
        )?;

        let options: BTreeSet<ColorGroup> = ColorGroup::iter().collect();
        let options: BTreeSet<ColorGroup> = &options - &item.get_color_set();

        let mut color_group = prompt::select_cmd(
            w,
            "Select a color group for which to add a location",
            &options,
        )?;

        if let ColorGroup::Other(_) = color_group {
            display::clear(w)?;
            let color_name = prompt::input_string(
                w,
                &format!("Enter the name of the color:"),
            )?;
            color_group = ColorGroup::Other(color_name);
        }

        display::clear(w)?;
        display::line(
            w,
            format!(
                "Adding a new color group to item with ID: {}",
                item.get_id()
            ),
        )?;

        let part_loc = prompt::input_string(
            w,
            &format!("Enter location of group {}:", color_group),
        )?;
        let part_loc = part_loc.to_uppercase();

        let mut new_item = item.clone();
        new_item.add_color_group(color_group, part_loc);
        Ok(Mode::EditItem {
            item: new_item,
            msg: None,
        })
    }

    fn remove_color_group<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: None } = &self.mode else {
            bail!(self, RemoveColorGroup);
        };

        display::clear(w)?;
        display::line(
            w,
            format!(
                "Removing a color group from item with ID: {}",
                item.get_id()
            ),
        )?;

        let options: BTreeSet<ColorGroup> = ColorGroup::iter().collect();
        let options: BTreeSet<ColorGroup> = &options & &item.get_color_set();

        let color_group =
            prompt::select_cmd(w, "Select color group to remove:", &options)?;
        let mut new_item = item.clone();
        new_item.remove_color_group(color_group);
        Ok(Mode::EditItem {
            item: new_item,
            msg: None,
        })
    }

    fn add_alt_id<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: _ } = &self.mode else {
            bail!(self, AddAltId);
        };

        display::clear(w)?;
        let new_id = prompt::input_u32(
            w,
            "Enter the new alternative part ID to add to this item",
        )?;

        if let Some(main_id) = self.db.contains_id(new_id) {
            let msg = Some(format!(
                "Item with part ID {} already exists in database under item with part ID {}",
                new_id, main_id
            ));
            return Ok(Mode::EditItem {
                item: item.clone(),
                msg,
            });
        }

        let mut new_item = item.clone();
        new_item.add_alt_id(new_id);
        Ok(Mode::EditItem {
            item: new_item,
            msg: None,
        })
    }

    fn remove_alt_id<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: _ } = &self.mode else {
            bail!(self, RemoveAltId);
        };

        display::clear(w)?;

        let options = item.get_alternative_ids().iter().map(|i| *i).collect();
        let alt_id = prompt::select_from_list(
            w,
            "Which alternative ID do you want to remove?",
            &options,
        )?;

        let mut new_item = item.clone();
        new_item.remove_alt_id(alt_id);
        Ok(Mode::EditItem {
            item: new_item,
            msg: None,
        })
    }

    fn delete_item<W: Write>(&mut self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item, msg: _ } = &self.mode else {
            bail!(self, DeleteItem);
        };

        display::clear(w)?;
        display::iter(w, item.to_string().split("\n"))?;

        let changes = format!(
            "Are you absolutely sure that you want to delete the item with ID: {}?\n",
            item.get_id(),
        );

        if prompt::confirmation(w, &changes)? {
            self.db.remove_item(item.get_id())?;
            Ok(Mode::Default {
                info: format!("Item with ID: {} was deleted.", item.get_id()),
            })
        } else {
            Ok(Mode::EditItem {
                item: item.clone(),
                msg: None,
            })
        }
    }
}