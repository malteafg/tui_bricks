use std::fmt;

use term_lib::cmd::{Cmd, CmdChar};

/// Appearence order is as the order is written in code.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Cmds {
    AddItem,
    DeleteItem,

    Quit,
    Edit,
    SaveEdit,
    QuitEdit,
    EditName,
    EditAmount,

    // MCmd(MultiCmd),
    AddColorGroup,
    AddAltId,

    RemoveColorGroup,
    RemoveAltId,

    SearchPartID,
    SearchName,
    SearchLocation,
}

impl CmdChar for Cmds {
    fn get_char(&self) -> char {
        match &self {
            Cmds::AddItem => 'a',
            Cmds::Quit => 'q',
            Cmds::Edit => 'e',
            Cmds::SaveEdit => 's',
            Cmds::QuitEdit => 'q',
            Cmds::EditName => 'n',
            Cmds::EditAmount => 'm',
            Cmds::DeleteItem => 'd',

            // MCmd(m_cmd) => m_cmd.get_char(),
            Cmds::AddColorGroup => 'c',
            Cmds::AddAltId => 'i',

            Cmds::RemoveColorGroup => 'c',
            Cmds::RemoveAltId => 'i',

            Cmds::SearchPartID => 'i',
            Cmds::SearchName => 'n',
            Cmds::SearchLocation => 'l',
        }
    }
}

impl Cmd for Cmds {
    fn get_info(&self) -> &'static str {
        match &self {
            Cmds::AddItem => "(a)dd a new item to the database",
            Cmds::Quit => "(q)uit the program",
            Cmds::Edit => "(e)dit the current item",
            Cmds::SaveEdit => "(s)ave the current changes and quit editing",
            Cmds::QuitEdit => {
                "(q)uit editing this part without saving changes currently made"
            }
            Cmds::EditName => "edit the (n)ame of this part",
            Cmds::EditAmount => "edit the a(m)ount of this part",
            Cmds::DeleteItem => "(d)elete the current item",
            // MCmd(m_cmd) => m_cmd.get_info(),
            Cmds::AddColorGroup => {
                "add a new (c)olor group and its location for this item"
            }
            Cmds::AddAltId => "add a new alternative (i)d for this item",

            Cmds::RemoveColorGroup => {
                "remove a (c)olor group and its location for this item"
            }
            Cmds::RemoveAltId => "remove an alternative (i)d for this item",

            Cmds::SearchPartID => "search by part (i)d",
            Cmds::SearchName => "search by (n)ame",
            Cmds::SearchLocation => "search by (l)ocation",
        }
    }
}

impl fmt::Display for Cmds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.get_char(), self.get_info())
    }
}

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
// pub enum MultiCmd {
//     SearchItem,
//     AddToItem,
//     RemoveFromItem,
// }

// impl CmdChar for MultiCmd {
//     fn get_char(&self) -> char {
//         use MultiCmd::*;
//         match &self {
//             SearchItem => 's',
//             AddToItem => 'a',
//             RemoveFromItem => 'r',
//         }
//     }
// }

// impl MultiCmd {
//     pub fn get_possible_cmds(&self) -> CmdList {
//         use MultiCmd::*;
//         match &self {
//             SearchItem => CmdList::new(vec![
//                 Cmd::SearchPartID,
//                 Cmd::SearchName,
//                 Cmd::SearchLocation,
//             ]),
//             AddToItem => CmdList::new(vec![Cmd::AddColorGroup, Cmd::AddAltId]),
//             RemoveFromItem => {
//                 CmdList::new(vec![Cmd::RemoveColorGroup, Cmd::RemoveAltId])
//             }
//         }
//     }

//     pub fn get_header(&self) -> &str {
//         use MultiCmd::*;
//         match &self {
//             SearchItem => "What do you want to search by?",
//             AddToItem => "What would you like to add to this item?",
//             RemoveFromItem => "What would you like to remove to this item?",
//         }
//     }

//     pub fn get_info(&self) -> &str {
//         use MultiCmd::*;
//         match &self {
//             SearchItem => "(s)earch for an item",
//             AddToItem => "(a)dd something to an item",
//             RemoveFromItem => "(r)emove something from an item",
//         }
//     }
// }

// impl fmt::Display for MultiCmd {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}: {}", self.get_char(), self.get_info())
//     }
// }
