use crossterm::{cursor, execute, queue};
use std::io::Write;

// use crate::command::EDIT;
use crate::command::Cmd;
use crate::data::{Database, Item, COMP_COLORS};
use crate::display;
use crate::display::EmitMode;
use crate::error::{Error, Result};
use crate::io;
use crate::mode::Mode;

pub struct State {
    db: Database,
    mode: Mode,
}

impl State {
    pub fn new() -> Result<Self> {
        let db = Database::new()?;
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

        let cmd_char = io::wait_for_char()?;
        let Some(cmd) = possible_cmds.get(cmd_char) else {
            self.mode = Mode::Default {
                info: "executing command failed".to_owned(),
            };
            return Ok(false);
        };

        use Cmd::*;
        self.mode = match cmd {
            Quit => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape)?;
                return Ok(true);
            }
            AddItem => self.add_item(w),
            SearchItem => self.search_item(w),
            Edit => self.edit_item(),
            CancelEdit => self.cancel_edit(w),
            SaveEdit => todo!(),
        }?;

        return Ok(false);
    }

    fn add_item<W: Write>(&mut self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        display::emit_line(w, "Adding a new item to the database")?;
        let part_id = display::input_u32(w, "Enter the part ID of the new item")?;

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
        Ok(Mode::DisplayItem { item: new_item })
    }

    fn search_item<W: Write>(&self, w: &mut W) -> Result<Mode> {
        display::clear(w)?;
        let searched_id = display::input_u32(w, "Enter the part ID of the new to search for.")?;

        if let Some(item) = self.db.get_item(searched_id) {
            return Ok(Mode::DisplayItem { item: item.clone() });
        }

        Ok(Mode::Default {
            info: format!("Part {} not found in database", searched_id),
        })
    }

    fn edit_item(&self) -> Result<Mode> {
        let Mode::DisplayItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };
        Ok(Mode::EditItem { item: item.clone() })
    }

    fn cancel_edit<W: Write>(&self, w: &mut W) -> Result<Mode> {
        let Mode::EditItem { item } = &self.mode else {
            return Err(Error::CmdModeMismatch { cmd: Cmd::Edit.to_string(), mode: self.mode.to_string() });
        };
        display::clear(w)?;
        if display::confirmation_prompt(w, "Are you sure you want to cancel changes?")? {
            Ok(Mode::DisplayItem { item: item.clone() })
        } else {
            Ok(Mode::EditItem { item: item.clone() })
        }
    }
}
