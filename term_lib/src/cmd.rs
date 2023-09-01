#[macro_export]
macro_rules! create_cmd_handler {
    ($name:ident, $cmd:ident, $mode:ident, $state:ident, $mode_trait:ident) => {
        use term_lib::display;
        use term_lib::input;

        pub trait $mode_trait {
            fn exec<W: std::io::Write>(
                &mut self,
                w: &mut W,
                state: &mut $state,
                cmd: $cmd,
            ) -> term_lib::Result<Option<$mode>>;

            fn emit_mode<W: std::io::Write>(
                &self,
                w: &mut W,
            ) -> term_lib::Result<()>;

            fn get_possible_cmds(&self) -> term_lib::cmd::CmdList<$cmd>;
        }

        pub struct $name {
            mode: $mode,
            state: $state,
        }

        impl $name {
            pub fn new(mode: $mode, state: $state) -> Self {
                Self { mode, state }
            }

            pub fn exec_cmd<W: std::io::Write>(
                &mut self,
                w: &mut W,
            ) -> Result<()> {
                use crossterm::{cursor, queue};

                display::clear(w)?;
                self.mode.emit_mode(w)?;

                let possible_cmds = self.mode.get_possible_cmds();
                display::emit_dash(w)?;
                display::emit_line(w, "List of possible commands:")?;
                queue!(w, cursor::MoveToNextLine(1))?;
                display::emit_iter(w, possible_cmds.iter())?;

                w.flush()?;

                let exec_cmd;
                loop {
                    match input::wait_for_cmdchar() {
                        Ok(cmd_char) => {
                            if let Some(cmd) = possible_cmds.get(cmd_char) {
                                exec_cmd = cmd;
                                break;
                            }
                        }
                        Err(term_lib::Error::Escape) => {}
                        Err(e) => return Err(e.into()),
                    }
                }

                if let Some(new_mode) =
                    self.mode.exec(w, &mut self.state, exec_cmd)?
                {
                    self.mode = new_mode;
                }
                Ok(())
            }
        }
    };
}

pub trait CmdChar {
    fn get_char(&self) -> char;
}

pub trait Cmd: Ord + PartialOrd + std::fmt::Display + Copy + CmdChar {
    fn get_info(&self) -> &'static str;
}

pub struct CmdList<C: Cmd> {
    cmds: Vec<C>,
}

impl<C: Cmd> CmdList<C> {
    pub fn new(mut cmds: Vec<C>) -> Self {
        cmds.sort();
        CmdList { cmds }
    }

    pub fn get(&self, char: char) -> Option<C> {
        for &cmd in &self.cmds {
            if cmd.get_char() == char {
                return Some(cmd);
            }
        }
        None
    }
}

impl<C: Cmd> core::ops::Deref for CmdList<C> {
    type Target = Vec<C>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.cmds
    }
}

impl<C: Cmd> core::ops::DerefMut for CmdList<C> {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.cmds
    }
}
