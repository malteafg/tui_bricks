mod mode;

use rebrickable_client::ClientDB;
use rebrickable_database::LocalDB;
use rebrickable_database_api::RebrickableDB;
use term_lib::{command::Command, display, input};

use std::io::Write;

use crate::mode::{Home, Mode};

pub struct DB;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Cmd {
    Search,
    Quit,
}

impl Command for Cmd {
    fn get_char(&self) -> char {
        use Cmd::*;
        match self {
            Search => 's',
            Quit => 'q',
        }
    }

    fn get_info(&self) -> &str {
        use Cmd::*;
        match self {
            Search => "(s)earch for an item",
            Quit => "(q)uit tui bricks",
        }
    }
}

fn run_with_rdb<RDB: RebrickableDB, W: Write>(rdb: RDB, w: &mut W) -> term_lib::Result<()> {
    let mut mode: Box<dyn Mode<RDB, W>> = Box::new(Home::new(None));

    let db = DB {};

    loop {
        display::clear(w)?;
        mode.render(w)?;
        let possible_cmds = mode.get_possible_cmds();
        display::dash(w)?;
        display::line(w, "List of possible commands:")?;
        display::newline(w, 1)?;
        display::possible_cmds(w, possible_cmds.iter())?;
        w.flush().unwrap();

        let cmd_char = match input::wait_for_cmdchar() {
            Ok('q') | Ok('Q') | Err(term_lib::Error::Quit) => return Ok(()),
            Ok(c) => c,
            Err(term_lib::Error::Escape) => continue,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
        };

        let Some(cmd) = possible_cmds
            .iter()
            .filter(|cmd| cmd.get_char() == cmd_char)
            .next()
        else {
            continue;
        };

        mode = mode.handle_cmd(*cmd, &db, &rdb);
    }
}

pub fn run() {
    let mut w = std::io::stdout();
    term_lib::init(&mut w).unwrap();
    match ClientDB::new() {
        Ok(rdb) => {
            run_with_rdb(rdb, &mut w).unwrap();
        }
        Err(_) => {
            let rdb = LocalDB::default();
            run_with_rdb(rdb, &mut w).unwrap();
        }
    };
    term_lib::quit(&mut w).unwrap();
}
