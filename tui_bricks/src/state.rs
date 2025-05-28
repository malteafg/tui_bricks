use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::path::PathBuf;

use strum::IntoEnumIterator;

use term_lib::command::Command;
use term_lib::{display, input, prompt};

use crate::cmd::{Cmd, MultiCmd};
use crate::data::{ColorGroup, Database, Item, LocSearch};
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

pub struct State<W: std::io::Write> {
    db: Database,
    mode: Mode,
    marker: PhantomData<W>,
}

impl<W: std::io::Write> State<W> {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let db = Database::new(db_path)?;
        let mode = Mode::Default { info: None };
        Ok(Self {
            db,
            mode,
            marker: PhantomData,
        })
    }

    pub fn wait_for_cmd(&mut self, w: &mut W) -> Result<()> {
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

        match new_mode {
            Ok(new_mode) => self.mode = new_mode,
            Err(Error::TermError(term_lib::Error::Escape)) => {}
            Err(Error::TermError(term_lib::Error::ParsingError(_))) => {}
            Err(err) => return Err(err),
        }

        Ok(())
    }

    fn handle_multi_cmd(&mut self, w: &mut W, m_cmd: MultiCmd) -> Result<Mode> {
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

    fn execute_cmd(&mut self, w: &mut W, cmd: Cmd) -> Result<Mode> {
        use Cmd::*;
        match cmd {
            AddItem => self.add_item(w),
            DeleteItem => self.delete_item(w),
            AddGroup => self.add_group(w),

            Bricklink => self.open_bricklink(),

            Quit => {
                return Err(Error::TermError(term_lib::Error::Quit));
            }
            Edit => self.edit_item(),

            QuitEdit => self.quit_edit(w),
            SaveEdit => self.save_edit(),
            EditName => self.edit_name(w),
            EditPartID => self.edit_part_id(w),
            MoveColorGroup => self.move_color_group(w),

            MCmd(m_cmd) => self.handle_multi_cmd(w, m_cmd),

            AddColorGroup => self.add_color_group(w),
            AddAltId => self.add_alt_id(w),

            RemoveColorGroup => self.remove_color_group(w),
            RemoveAltId => self.remove_alt_id(w),

            SearchPartID => self.search_by_id(w),
            SearchName => self.search_by_name(),
            SearchLocation => self.search_by_location(w),

            ViewStats => self.view_stats(),
            QuitStats => self.quit_stats(),
        }
    }

    fn add_item(&mut self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        display::line(w, "Adding a new item to the database")?;
        let part_id = prompt::input_u32(w, "Enter the part ID of the new item")?;

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
        let name = prompt::input_string(w, &format!("Enter name of new part: {}", part_id))?;

        let new_item = Item::new(part_id, name);
        self.db.add_item(new_item.clone())?;
        Ok(Mode::EditItem {
            old_item: new_item.clone(),
            new_item,
            msg: None,
        })
    }

    fn add_group(&mut self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        display::line(w, "Adding a new group to the database")?;

        let part_id = self.db.get_next_group_id();

        display::clear(w)?;
        display::line(w, "Adding a new group to the database")?;
        let name = prompt::input_string(w, "Enter name of new group:")?;

        let new_item = Item::new(part_id, name);
        self.db.add_item(new_item.clone())?;
        Ok(Mode::EditItem {
            old_item: new_item.clone(),
            new_item,
            msg: None,
        })
    }

    fn search_by_id(&self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        let searched_id = prompt::input_u32(w, "Enter the part ID of the new to search for.")?;

        if let Ok(item) = self.db.get_item_by_id(searched_id) {
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg: None,
            });
        }

        Ok(Mode::Default {
            info: Some(format!("Part {} not found in database", searched_id)),
        })
    }

    fn search_by_name(&self) -> Result<Mode> {
        let opts = self.db.get_all_names_string();

        let searched_name = prompt::fzf_search(&opts)?;
        if searched_name.is_empty() {
            return Ok(self.mode.clone());
        }

        if let Ok(item) = self.db.get_item_by_name(&searched_name) {
            return Ok(Mode::DisplayItem {
                item: item.clone(),
                msg: None,
            });
        }

        Ok(Mode::Default {
            info: Some(format!("Part {} not found in database", searched_name)),
        })
    }

    fn search_by_location(&self, w: &mut W) -> Result<Mode> {
        let opts = self.db.get_all_locations_string();
        let searched_loc = prompt::fzf_search(&opts)?;
        if !self.db.contains_location(&searched_loc) {
            return Ok(self.mode.clone());
        }

        let locations = self.db.get_items_at_location(&searched_loc);

        display::clear(w)?;

        let info = format!("List of items located at location:\n{}", searched_loc);
        display::header(w, &info)?;
        let selected: LocSearch = prompt::select_from_list(w, None, locations)?;

        let item = self.db.get_item_by_id(selected.id)?;

        Ok(Mode::DisplayItem {
            item: item.clone(),
            msg: None,
        })
    }

    fn edit_item(&self) -> Result<Mode> {
        let Mode::DisplayItem { item, msg: _ } = &self.mode else {
            bail!(self, Edit);
        };
        Ok(Mode::EditItem {
            old_item: item.clone(),
            new_item: item.clone(),
            msg: None,
        })
    }

    fn save_edit(&mut self) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: _,
        } = &self.mode
        else {
            bail!(self, SaveEdit);
        };
        self.db.update_item(old_item, new_item)?;
        Ok(Mode::DisplayItem {
            item: new_item.clone(),
            msg: None,
        })
    }

    fn quit_edit(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: _,
        } = &self.mode
        else {
            bail!(self, QuitEdit);
        };

        if old_item == new_item {
            return Ok(Mode::DisplayItem {
                item: old_item.clone(),
                msg: None,
            });
        }

        display::clear(w)?;
        let changes = format!(
            "Are you sure you want to quit editing and cancel these changes?\n\n{}",
            old_item.diff(new_item)
        );

        if prompt::confirmation(w, &changes)? {
            Ok(Mode::DisplayItem {
                item: new_item.clone(),
                msg: None,
            })
        } else {
            Ok(Mode::EditItem {
                old_item: old_item.clone(),
                new_item: new_item.clone(),
                msg: None,
            })
        }
    }

    fn edit_name(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: None,
        } = &self.mode
        else {
            bail!(self, EditName);
        };

        display::clear(w)?;
        display::line(w, format!("Editing name of part: {}", new_item.get_id()))?;
        let new_name = prompt::edit_string(w, "Enter new name:", new_item.get_name())?;

        if let Some(existing_id) = self.db.contains_name(&new_name) {
            return Ok(Mode::EditItem {
                old_item: old_item.clone(),
                new_item: new_item.clone(),
                msg: Some(format!(
                    "The item with part ID {} already has the name {}",
                    existing_id, new_name,
                )),
            });
        }

        let mut updated_item = new_item.clone();
        updated_item.set_name(&new_name);
        Ok(Mode::EditItem {
            old_item: old_item.clone(),
            new_item: updated_item,
            msg: Some("Item name succesfully updated.".to_string()),
        })
    }

    fn edit_part_id(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: None,
        } = &self.mode
        else {
            bail!(self, EditName);
        };

        display::clear(w)?;
        display::line(w, format!("Editing ID of part: {}", new_item.get_id()))?;
        let new_id = prompt::edit_u32(w, "Enter new ID:", new_item.get_id())?;

        if let Some(main_id) = self.db.contains_id(new_id) {
            return Ok(Mode::EditItem {
                old_item: old_item.clone(),
                new_item: new_item.clone(),
                msg: Some(format!(
                    "Item with part ID {} already exists in database under item with part ID {}",
                    new_id, main_id
                )),
            });
        }

        let mut updated_item = new_item.clone();
        updated_item.set_id(new_id);
        Ok(Mode::EditItem {
            old_item: old_item.clone(),
            new_item: updated_item,
            msg: Some("Part ID succesfully updated.".to_string()),
        })
    }

    fn move_color_group(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: None,
        } = &self.mode
        else {
            bail!(self, EditName);
        };

        display::clear(w)?;

        let color_group: &ColorGroup = if new_item.get_locations().len() == 1 {
            &new_item.get_locations()[0].0
        } else {
            prompt::select_from_list(
                w,
                Some("Select color group to move:"),
                new_item.get_color_set().iter(),
            )?
        };

        display::clear(w)?;
        display::line(
            w,
            format!(
                "Moving color group {} from item with ID: {}",
                color_group,
                new_item.get_id()
            ),
        )?;
        let part_loc =
            prompt::input_string(w, &format!("Enter new location of group {}:", color_group))?;
        let part_loc = part_loc.to_uppercase();

        let mut updated_item = new_item.clone();
        updated_item.remove_color_group(color_group);
        updated_item.add_color_group(color_group.clone(), part_loc);

        Ok(Mode::EditItem {
            old_item: old_item.clone(),
            new_item: updated_item,
            msg: None,
        })
    }

    fn add_color_group(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: _,
        } = &self.mode
        else {
            bail!(self, AddColorGroup)
        };

        display::clear(w)?;
        display::line(
            w,
            format!(
                "Adding a new color group to item with ID: {}",
                new_item.get_id()
            ),
        )?;

        let color_set = new_item.get_color_set();
        let options = ColorGroup::iter().filter(|c| !color_set.contains(c));

        let mut color_group = prompt::select_cmd(
            w,
            "Select a color group for which to add a location",
            options,
        )?;

        if let ColorGroup::Other(_) = color_group {
            display::clear(w)?;

            let create_new = "Create new color group".to_owned();

            let item_o_color_set = new_item.get_other_color_set();
            let options = self
                .db
                .get_other_color_set()
                .iter()
                .filter(|s| !item_o_color_set.contains(*s));

            let color_name = prompt::select_from_list(
                w,
                Some("Select other color group or create a new one"),
                std::iter::once(&create_new).chain(options),
            )?;

            if color_name == &create_new {
                display::clear(w)?;
                let color_name =
                    prompt::input_string(w, &format!("Enter the name of the new color group:"))?;
                color_group = ColorGroup::Other(color_name);
            } else {
                color_group = ColorGroup::Other(color_name.to_string());
            }
        }

        display::clear(w)?;
        display::line(
            w,
            format!(
                "Adding a new color group to item with ID: {}",
                new_item.get_id()
            ),
        )?;

        let part_loc =
            prompt::input_string(w, &format!("Enter location of group {}:", color_group))?;
        let part_loc = part_loc.to_uppercase();

        let mut updated_item = new_item.clone();
        updated_item.add_color_group(color_group, part_loc);
        Ok(Mode::EditItem {
            old_item: old_item.clone(),
            new_item: updated_item,
            msg: None,
        })
    }

    fn remove_color_group(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: None,
        } = &self.mode
        else {
            bail!(self, RemoveColorGroup);
        };

        display::clear(w)?;
        display::line(
            w,
            format!(
                "Removing a color group from item with ID: {}",
                new_item.get_id()
            ),
        )?;

        let color_group: &ColorGroup = prompt::select_from_list(
            w,
            Some("Select color group to remove:"),
            new_item.get_color_set().iter(),
        )?;

        let mut updated_item = new_item.clone();
        updated_item.remove_color_group(color_group);
        Ok(Mode::EditItem {
            old_item: old_item.clone(),
            new_item: updated_item,
            msg: None,
        })
    }

    fn add_alt_id(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: _,
        } = &self.mode
        else {
            bail!(self, AddAltId);
        };

        display::clear(w)?;
        let new_id = prompt::input_u32(w, "Enter the new alternative part ID to add to this item")?;

        if let Some(main_id) = self.db.contains_id(new_id) {
            let msg = Some(format!(
                "Item with part ID {} already exists in database under item with part ID {}",
                new_id, main_id
            ));
            return Ok(Mode::EditItem {
                old_item: old_item.clone(),
                new_item: new_item.clone(),
                msg,
            });
        }

        let mut updated_item = new_item.clone();
        updated_item.add_alt_id(new_id);
        Ok(Mode::EditItem {
            old_item: old_item.clone(),
            new_item: updated_item,
            msg: None,
        })
    }

    fn remove_alt_id(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: _,
        } = &self.mode
        else {
            bail!(self, RemoveAltId);
        };

        display::clear(w)?;

        let options: BTreeSet<u32> = new_item.get_alternative_ids().iter().map(|i| *i).collect();
        let alt_id = prompt::select_from_list(
            w,
            Some("Which alternative ID do you want to remove?"),
            options.iter(),
        )?;

        let mut updated_item = new_item.clone();
        updated_item.remove_alt_id(*alt_id);
        Ok(Mode::EditItem {
            old_item: old_item.clone(),
            new_item: updated_item,
            msg: None,
        })
    }

    fn delete_item(&mut self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem {
            old_item,
            new_item,
            msg: _,
        } = &self.mode
        else {
            bail!(self, DeleteItem);
        };

        display::clear(w)?;
        display::iter(w, old_item.to_string().split("\n"))?;

        let changes = format!(
            "Are you absolutely sure that you want to delete the item with ID: {}?\n",
            old_item.get_id(),
        );

        if prompt::confirmation(w, &changes)? {
            self.db.remove_item(old_item.get_id())?;
            Ok(Mode::Default {
                info: Some(format!("Item with ID: {} was deleted.", old_item.get_id())),
            })
        } else {
            Ok(Mode::EditItem {
                old_item: old_item.clone(),
                new_item: new_item.clone(),
                msg: None,
            })
        }
    }

    fn view_stats(&self) -> Result<Mode> {
        let stats = self.db.get_stats();
        Ok(Mode::ViewStatistics { stats })
    }

    fn quit_stats(&self) -> Result<Mode> {
        Ok(Mode::Default { info: None })
    }

    fn open_bricklink(&self) -> Result<Mode> {
        match &self.mode {
            Mode::EditItem { new_item: item, .. } | Mode::DisplayItem { item, .. } => {
                webbrowser::open(&format!(
                    "https://www.bricklink.com/v2/catalog/catalogitem.page?P={}",
                    item.get_id()
                ))?;
            }
            _ => {
                bail!(self, Bricklink);
            }
        }
        Ok(self.mode.clone())
    }
}
