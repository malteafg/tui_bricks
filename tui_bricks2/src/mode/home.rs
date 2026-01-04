use rebrickable_database_api::RebrickableDB;
use term_lib::{command::CmdList, display};

use std::io::Write;

use crate::{Cmd, DB, mode::Mode};

pub struct Home {
    info: Option<String>,
}

impl Home {
    pub fn new(info: Option<String>) -> Self {
        Self { info }
    }
}

impl<RDB: RebrickableDB, W: Write> Mode<RDB, W> for Home {
    fn get_possible_cmds(&self) -> CmdList<Cmd> {
        CmdList::new(vec![Cmd::Search, Cmd::Quit])
    }

    fn handle_cmd(
        self: Box<Self>,
        cmd: Cmd,
        db: &DB,
        rebrickable_db: &RDB,
    ) -> Box<dyn Mode<RDB, W>> {
        match cmd {
            Cmd::Search => unimplemented!(),
            _ => self,
        }
    }

    fn render(&self, w: &mut W) -> term_lib::Result<()> {
        display::header(w, "Welcome to tui_bricks")?;
        if let Some(info) = &self.info {
            display::iter(w, info.split("\n"))?;
        }
        Ok(())
    }
}
