use crate::Result;

trait Cmd {
    fn get_char(&self) -> char;
}

pub struct CmdTest;
pub struct Cmd2;

impl Cmd for CmdTest {
    fn get_char(&self) -> char {
        'q'
    }
}

impl Cmd for Cmd2 {
    fn get_char(&self) -> char {
        'r'
    }
}

trait Mode<T, C: Cmd> {
    fn emit_mode(&self) -> Result<()>;

    fn exec(
        &mut self,
        w: Box<dyn std::io::Write>,
        state: &mut T,
        cmd: C,
    ) -> Option<Box<Modes>>;
}

pub struct TestMode {
    item: String,
}

struct State;

// impl Mode<State> for TestMode {
//     fn emit_mode(&self) -> Result<()> {
//         Ok(())
//     }
//     fn exec(
//         &mut self,
//         w: Box<dyn std::io::Write>,
//         state: &mut State,
//         cmd: Cmd,
//     ) -> Option<Box<dyn Mode<State>>> {
//         None
//     }
// }

// trait Exec<T, W: std::io::Write> {
//     type Mode;

//     fn exec(
//         &mut self,
//         w: &mut W,
//         state: &mut T,
//         cmd: Cmd,
//     ) -> Option<Box<dyn Mode<T>>>;
// }

/// Takes in a name and all the modes
// macro_rules! create_cmd_handler {
//     () => {
//
//     };
// }

// impl<T, W: std::io::Write> Exec<T, W> for CmdConnector<T, TestMode> {
//     type Mode = TestMode;

//     fn exec(
//         &mut self,
//         w: &mut W,
//         state: &mut T,
//         cmd: Cmd,
//     ) -> Option<Box<dyn Mode<T>>> {
//         None
//     }
// }

pub enum Modes {
    Mode1(TestMode),
    Mode2(TestMode),
}

pub enum Cmds {
    Cmd1(CmdTest),
    Cmd2(Cmd2),
}

pub struct CmdHandler<T> {
    mode: Modes,
    state: T,
}

impl<T> CmdHandler<T> {
    pub fn new(mode: Modes, state: T) -> Self {
        Self { mode, state }
    }

    pub fn exec_cmd<W: std::io::Write>(&mut self, w: &mut W) -> Result<()> {
        match &self.mode {
            Modes::Mode1(mode) => Ok(()),
            Modes::Mode2(mode) => Ok(()),
        }
    }
}
