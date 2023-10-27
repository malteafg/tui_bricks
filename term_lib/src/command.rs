pub trait Command {
    fn get_char(&self) -> char;
    fn get_info(&self) -> &str;

    fn display_as_cmd(&self) -> String {
        format!("{}: {}", self.get_char(), self.get_info())
    }
}

pub struct CmdList<C: Command + Ord> {
    cmds: Vec<C>,
}

impl<C: Command + Ord> CmdList<C> {
    pub fn new(mut cmds: Vec<C>) -> Self {
        cmds.sort();
        CmdList { cmds }
    }

    pub fn get(&self, char: char) -> Option<&C> {
        for cmd in &self.cmds {
            if cmd.get_char() == char {
                return Some(cmd);
            }
        }
        None
    }
}

impl<C: Command + Ord> core::ops::Deref for CmdList<C> {
    type Target = Vec<C>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.cmds
    }
}

impl<C: Command + Ord> core::ops::DerefMut for CmdList<C> {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.cmds
    }
}

// pub trait Cmd: Ord + PartialOrd + std::fmt::Display + Copy {
//     fn get_char(&self) -> char;
//     fn get_info(&self) -> &'static str;
// }

// pub trait DisplayCmd {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
// }

// pub trait Mode<C: Cmd, T, W: std::io::Write = Stdout> {
//     fn exec(&mut self, state: &mut T, cmd: C)
//         -> Option<Box<dyn Mode<C, T, W>>>;
//     fn emit_mode(&self, w: &mut W) -> crate::Result<()>;
//     fn get_possible_cmds(&self) -> CmdList<C>;
// }

// pub struct CmdHandler<C: Cmd, T, W: std::io::Write = Stdout> {
//     mode: Box<dyn Mode<C, T, W>>,
//     state: T,
// }

// impl<C: Cmd, T, W: std::io::Write> CmdHandler<C, T, W> {
//     pub fn new(mode: Box<dyn Mode<C, T, W>>, state: T) -> Self {
//         Self { mode, state }
//     }

//     pub fn exec_cmd(&mut self, w: &mut W) -> Result<()> {
//         let possible_cmds = self.mode.get_possible_cmds();
//         display::emit_dash(w)?;
//         display::emit_line(w, "List of possible commands:")?;
//         queue!(w, cursor::MoveToNextLine(1))?;
//         display::emit_iter(w, possible_cmds.iter())?;

//         w.flush()?;

//         let exec_cmd;
//         loop {
//             match input::wait_for_cmdchar() {
//                 Ok(cmd_char) => {
//                     if let Some(cmd) = possible_cmds.get(cmd_char) {
//                         exec_cmd = cmd;
//                         break;
//                     }
//                 }
//                 Err(crate::Error::Escape) => {}
//                 Err(e) => return Err(e.into()),
//             }
//         }

//         if let Some(new_mode) = self.mode.exec(&mut self.state, exec_cmd) {
//             self.mode = new_mode;
//         }

//         display::clear(w)?;
//         self.mode.emit_mode(w)?;
//         Ok(())
//     }
// }
