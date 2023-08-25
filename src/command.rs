use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AddToItem {
    ColorGroup,
    AlternativeId,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Search {
    PartID,
    Name,
    Location,
}

/// Appearence order is as the order is written in code.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Cmd {
    AddItem,
    SearchItem,
    // SearchItem(Search),
    Quit,
    Edit,
    SaveEdit,
    QuitEdit,
    AddColorGroup,
    RemoveColorGroup,
    EditName,
    EditAmount,
    DeleteItem,
}

impl Cmd {
    pub fn get_char(&self) -> char {
        use Cmd::*;
        match &self {
            AddItem => 'a',
            SearchItem => 'p',
            Quit => 'q',
            Edit => 'e',
            SaveEdit => 's',
            QuitEdit => 'q',
            AddColorGroup => 'a',
            RemoveColorGroup => 'r',
            EditName => 'n',
            EditAmount => 'm',
            DeleteItem => 'd',
        }
    }

    pub fn get_info(&self) -> &str {
        use Cmd::*;
        match &self {
            AddItem => "(a)dd a new item to the menu",
            SearchItem => "search for the given (p)art by its id",
            Quit => "(q)uit the program",
            Edit => "(e)dit the current item",
            SaveEdit => "(s)ave the current changes and quit editing",
            QuitEdit => "(q)uit editing this part without saving changes currently made",
            AddColorGroup => "(a)dd a new color group and its location for this item",
            RemoveColorGroup => "(r)emove a color group and its location for this item",
            EditName => "edit the (n)ame of this part",
            EditAmount => "edit the a(m)ount of this part",
            DeleteItem => "(d)elete the current item",
        }
    }
}

impl fmt::Display for Cmd {
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
