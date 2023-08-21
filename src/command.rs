use std::fmt::Display;

pub const ADD_ITEM: Cmd = Cmd {
    char: 'a',
    info: "(a)dd a new item to the menu",
};

pub const SEARCH_ITEM: Cmd = Cmd {
    char: 'p',
    info: "search for the given (p)art by its id",
};

#[derive(Debug, PartialEq, Eq)]
pub struct Cmd {
    char: char,
    info: &'static str,
}

// pub trait Cmd {
//     fn get_char() -> char;
//     fn get_info() -> String;
//     fn exec<W: Write>(w: W, state: &mut State) -> Result<Mode>;
// }

impl Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.char, self.info)
    }
}

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
    cmds: Vec<Cmd>,
}

impl CmdList {
    pub fn new(mut cmds: Vec<Cmd>) -> Self {
        cmds.sort();
        CmdList { cmds }
    }
}

impl core::ops::Deref for CmdList {
    type Target = Vec<Cmd>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.cmds
    }
}

impl core::ops::DerefMut for CmdList {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.cmds
    }
}
