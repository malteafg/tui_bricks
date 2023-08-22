use crossterm::{cursor, execute, queue};

use crate::command::CmdList;
use crate::data::{Database, Item, COMP_COLORS};
use crate::display;
use crate::display::EmitMode;
use crate::error::Result;
use crate::io;

pub struct State {
    db: Database,
    mode: Mode,
}

pub enum Mode {
    Default { info: String },
    DisplayItem { item: Item },
}

impl Mode {
    pub fn get_possible_cmds(&self) -> CmdList {
        use crate::command::*;
        use Mode::*;
        match self {
            Default { .. } => CmdList::new(vec![ADD_ITEM, SEARCH_ITEM]),
            DisplayItem { .. } => CmdList::new(vec![ADD_ITEM, SEARCH_ITEM]),
        }
    }
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
        display::clear(w)?;
        display::header(w)?;

        self.mode.emit_mode(w)?;

        let possible_cmds = self.mode.get_possible_cmds();
        queue!(w, cursor::MoveToNextLine(1))?;
        display::emit_line(w, "List of possible commands:")?;
        queue!(w, cursor::MoveToNextLine(1))?;
        display::emit_iter(w, possible_cmds.iter())?;

        w.flush()?;

        match io::wait_for_char()? {
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape)?;
                return Ok(true);
            }
            c => {
                self.mode = self.execute_cmd(c, w)?;
                return Ok(false);
            }
        };
    }

    fn execute_cmd<W: std::io::Write>(&mut self, char: char, w: &mut W) -> Result<Mode> {
        match char {
            'a' => self.add_part(w),
            'p' => self.search_part(w),
            _ => Ok(Mode::Default {
                info: "executing command failed".to_owned(),
            }),
        }
    }

    fn add_part<W: std::io::Write>(&mut self, w: &mut W) -> Result<Mode> {
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

    fn search_part<W>(&self, w: &mut W) -> Result<Mode>
    where
        W: std::io::Write,
    {
        display::clear(w)?;
        let searched_id = display::input_u32(w, "Enter the part ID of the new to search for.")?;

        if let Some(item) = self.db.get_item(searched_id) {
            return Ok(Mode::DisplayItem { item: item.clone() });
        }

        Ok(Mode::Default {
            info: format!("Part {} not found in database", searched_id),
        })
    }
}
