use std::fmt::Display;
use std::io::Write;

use crate::error::Result;
use crate::mode::Mode;

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

fn unit_exec(w: &mut dyn Write) -> Result<Mode> {
    Ok(Mode::Default {
        info: "unit_exec".to_string(),
    })
}

// #[derive(Debug)]
pub struct Cmd {
    char: char,
    info: &'static str,
    exec: fn(&mut dyn Write) -> Result<Mode>,
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
