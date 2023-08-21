use crossterm::{
    cursor, execute, queue,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};

use crate::command::CmdList;
use crate::data::{ColorGroup, Database, Item, COMP_COLORS};
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
        queue!(
            w,
            ResetColor,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print("Enter ID of new part"),
            cursor::MoveToNextLine(1),
            cursor::Show,
        )?;
        w.flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let part_id: u32 = input.trim().parse()?;

        queue!(
            w,
            ResetColor,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print("Select a color group by typing its first letter"),
            cursor::MoveToNextLine(1),
            Print("(you can add more later)"),
            cursor::MoveToNextLine(1),
            cursor::Hide,
        )?;
        w.flush()?;

        for (i, color) in COMP_COLORS.iter().enumerate() {
            queue!(
                w,
                Print(format!("{}: {}", i, color)),
                cursor::MoveToNextLine(1),
            )?;
        }
        w.flush()?;

        use ColorGroup::*;
        let color_group = match io::wait_for_char()? {
            'a' => All,
            'b' => Basic,
            'n' => Nature,
            'g' => Grey,
            'r' => Road,
            't' => Translucent,
            _ => todo!(),
        };

        queue!(
            w,
            ResetColor,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print(format!("Enter location of group {}:", color_group)),
            cursor::MoveToNextLine(1),
            cursor::Show,
        )?;
        w.flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let part_loc = input.trim();

        let new_item = Item::new(part_id, color_group, part_loc.to_owned());

        self.db.add_item(new_item.clone())?;

        Ok(Mode::DisplayItem { item: new_item })
    }

    fn search_part<W>(&self, w: &mut W) -> Result<Mode>
    where
        W: std::io::Write,
    {
        queue!(
            w,
            ResetColor,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print("Search for part by ID:"),
            cursor::MoveToNextLine(1),
            cursor::Show,
        )?;
        w.flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let searched_id: u32 = input.trim().parse()?;

        if let Some(item) = self.db.get_item(searched_id) {
            return Ok(Mode::DisplayItem { item: item.clone() });
        }

        Ok(Mode::Default {
            info: format!("Part {} not found in database", searched_id),
        })
    }
}
