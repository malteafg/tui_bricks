use std::fmt::Display;
use std::io::Write;

use crate::display;
use crate::error::{Error, Result};
use crate::mode::Mode;
use crate::state::State;

pub static ADD_ITEM: Cmd = Cmd {
    char: 'a',
    info: "(a)dd a new item to the menu",
    exec: unit_exec,
};

pub const SEARCH_ITEM: Cmd = Cmd {
    char: 'p',
    info: "search for the given (p)art by its id",
    exec: unit_exec,
};

pub const QUIT: Cmd = Cmd {
    char: 'q',
    info: "(q)uit the program",
    exec: unit_exec,
};

pub const EDIT: Cmd = Cmd {
    char: 'e',
    info: "(e)dit the current item",
    exec: unit_exec,
};

pub const SAVE_EDIT: Cmd = Cmd {
    char: 's',
    info: "(s)ave the current changes",
    exec: unit_exec,
};

pub static CANCEL_EDIT: Cmd = Cmd {
    char: 'c',
    info: "(c)ancel editing without saving changes currently made",
    exec: unit_exec,
};

fn unit_exec(w: &mut dyn Write, state: &mut State) -> Result<Mode> {
    Ok(Mode::Default {
        info: "unit_exec".to_string(),
    })
}

// fn add_item(w: &mut dyn Write, state: &mut State, mode: Mode) -> Result<Mode> {
//     display::clear(w)?;
//     display::emit_line(w, "Adding a new item to the database")?;
//     let part_id = display::input_u32(w, "Enter the part ID of the new item")?;

//     display::clear(w)?;
//     display::emit_line(w, "Adding a new item to the database")?;
//     let color_group = display::select_from_list(
//         w,
//         "Select a color group by typing its first letter\n(you can add more groups later)",
//         &COMP_COLORS,
//     )?;

//     display::clear(w)?;
//     display::emit_line(w, "Adding a new item to the database")?;
//     let part_loc = display::input_string(w, &format!("Enter location of group {}:", color_group))?;

//     let new_item = Item::new(part_id, color_group, part_loc.to_owned());
//     self.db.add_item(new_item.clone())?;
//     Ok(Mode::DisplayItem { item: new_item })
// }

// fn search_item(w: &mut dyn Write, state: &mut State, mode: Mode) -> Result<Mode> {
//     display::clear(w)?;
//     let searched_id = display::input_u32(w, "Enter the part ID of the new to search for.")?;

//     if let Some(item) = self.db.get_item(searched_id) {
//         return Ok(Mode::DisplayItem { item: item.clone() });
//     }

//     Ok(Mode::Default {
//         info: format!("Part {} not found in database", searched_id),
//     })
// }

// fn edit_item(w: &mut dyn Write, state: &mut State, mode: Mode) -> Result<Mode> {
//     let Mode::DisplayItem { item } = &self.mode else {
//             return Err(Error::CmdModeMismatch { cmd: EDIT.to_string(), mode: self.mode.to_string() });
//         };
//     Ok(Mode::EditItem { item: item.clone() })
// }

// fn cancel_edit(w: &mut dyn Write, state: &mut State, mode: Mode) -> Result<Mode> {
//     let Mode::EditItem { item } = &self.mode else {
//             return Err(Error::CmdModeMismatch { cmd: EDIT.to_string(), mode: self.mode.to_string() });
//         };
//     display::clear(w)?;
//     if display::confirmation_prompt(w, "Are you sure you want to cancel changes?")? {
//         Ok(Mode::DisplayItem { item: item.clone() })
//     } else {
//         Ok(Mode::EditItem { item: item.clone() })
//     }
// }

pub struct Cmd {
    char: char,
    info: &'static str,
    exec: fn(&mut dyn Write, &mut State) -> Result<Mode>,
}

impl Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.char, self.info)
    }
}

impl PartialEq for Cmd {
    // Implementing the partial equality comparison
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char
    }
}

impl Eq for Cmd {}

impl Ord for Cmd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.char.cmp(&other.char)
    }
}

impl PartialOrd for Cmd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.char.cmp(&other.char))
    }
}

pub struct CmdList {
    cmds: Vec<&'static Cmd>,
}

impl CmdList {
    pub fn new(mut cmds: Vec<&'static Cmd>) -> Self {
        cmds.sort();
        CmdList { cmds }
    }
}

impl core::ops::Deref for CmdList {
    type Target = Vec<&'static Cmd>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.cmds
    }
}

impl core::ops::DerefMut for CmdList {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.cmds
    }
}
