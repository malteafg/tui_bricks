use std::fmt;

/// Appearence order is as the order is written in code.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Cmd {
    AddItem,
    DeleteItem,

    Quit,
    Edit,
    SaveEdit,
    QuitEdit,
    EditName,
    EditAmount,

    MCmd(MultiCmd),

    AddColorGroup,
    AddAltId,

    RemoveColorGroup,
    RemoveAltId,

    SearchPartID,
    SearchName,
    SearchLocation,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MultiCmd {
    SearchItem,
    AddToItem,
    RemoveFromItem,
}

impl Cmd {
    pub fn get_char(&self) -> char {
        use Cmd::*;
        match &self {
            AddItem => 'a',
            Quit => 'q',
            Edit => 'e',
            SaveEdit => 's',
            QuitEdit => 'q',
            EditName => 'n',
            EditAmount => 'm',
            DeleteItem => 'd',

            MCmd(m_cmd) => m_cmd.get_char(),

            AddColorGroup => 'c',
            AddAltId => 'i',

            RemoveColorGroup => 'c',
            RemoveAltId => 'i',

            SearchPartID => 'i',
            SearchName => 'n',
            SearchLocation => 'l',
        }
    }

    pub fn get_info(&self) -> &str {
        use Cmd::*;
        match &self {
            AddItem => "(a)dd a new item to the menu",
            Quit => "(q)uit the program",
            Edit => "(e)dit the current item",
            SaveEdit => "(s)ave the current changes and quit editing",
            QuitEdit => "(q)uit editing this part without saving changes currently made",
            EditName => "edit the (n)ame of this part",
            EditAmount => "edit the a(m)ount of this part",
            DeleteItem => "(d)elete the current item",
            MCmd(m_cmd) => m_cmd.get_info(),

            AddColorGroup => "add a new (c)olor group and its location for this item",
            AddAltId => "add a new alternative (i)d for this item",

            RemoveColorGroup => "remove a (c)olor group and its location for this item",
            RemoveAltId => "remove an alternative (i)d for this item",

            SearchPartID => "search by part (i)d",
            SearchName => "search by (n)ame",
            SearchLocation => "search by (l)ocation",
        }
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.get_char(), self.get_info())
    }
}

impl MultiCmd {
    pub fn get_possible_cmds(&self) -> CmdList {
        use MultiCmd::*;
        match &self {
            SearchItem => CmdList::new(vec![
                Cmd::SearchPartID,
                Cmd::SearchName,
                Cmd::SearchLocation,
            ]),
            AddToItem => CmdList::new(vec![Cmd::AddColorGroup, Cmd::AddAltId]),
            RemoveFromItem => CmdList::new(vec![Cmd::RemoveColorGroup, Cmd::RemoveAltId]),
        }
    }

    pub fn get_header(&self) -> &str {
        use MultiCmd::*;
        match &self {
            SearchItem => "What do you want to search by?",
            AddToItem => "What would you like to add to this item?",
            RemoveFromItem => "What would you like to remove to this item?",
        }
    }

    pub fn get_char(&self) -> char {
        use MultiCmd::*;
        match &self {
            SearchItem => 's',
            AddToItem => 'a',
            RemoveFromItem => 'r',
        }
    }

    pub fn get_info(&self) -> &str {
        use MultiCmd::*;
        match &self {
            SearchItem => "(s)earch for an item",
            AddToItem => "(a)dd something to an item",
            RemoveFromItem => "(r)emove something from an item",
        }
    }
}

impl fmt::Display for MultiCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.get_char(), self.get_info())
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

    pub fn get(&self, char: char) -> Option<Cmd> {
        for &cmd in &self.cmds {
            if cmd.get_char() == char {
                return Some(cmd);
            }
        }
        None
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
