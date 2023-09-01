extern crate log;

use std::path::PathBuf;

use crossterm::{cursor, execute, style, terminal};

use crate::cmd::Cmds;
use crate::data::Database;
use crate::error::{Error, Result};
use crate::mode::Modes;
// use tui_bricks::state::State;

pub mod cmd;
pub mod data;
pub mod error;
pub mod io;
pub mod mode;
// pub mod state;

#[cfg(not(debug_assertions))]
pub mod config;

#[cfg(not(debug_assertions))]
fn get_user_db_path() -> Result<PathBuf> {
    use figment::{
        providers::{Format, Serialized, Yaml},
        Figment,
    };
    use tui_bricks::config::Config;

    let mut config_path = tui_bricks::io::get_config_dir();
    config_path.push("config.yml");

    let config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Yaml::file(config_path))
        .extract()
        .unwrap();

    Ok(config.get_db_path())
}

fn quit<W: std::io::Write>(w: &mut W) -> Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    Ok(())
}

term_lib::create_cmd_handler!(CmdHandler, Cmds, Modes, Database, Mode);

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: std::io::Write,
{
    #[cfg(not(debug_assertions))]
    let db_path = get_user_db_path()?;

    #[cfg(debug_assertions)]
    let db_path = PathBuf::new().join("test_db.yml");

    let db = Database::new(db_path)?;
    let mode = Modes::Default {
        info: "Type any of the following characters to execute the associated command"
            .to_owned(),
    };

    let mut cmd_handler: CmdHandler = CmdHandler::new(mode, db);

    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        match cmd_handler.exec_cmd(w) {
            Ok(()) => {}
            Err(Error::TermError(term_lib::Error::Quit)) => {
                quit(w)?;
                return Ok(());
            }
            Err(e) => {
                quit(w)?;
                return Err(e);
            }
        }
    }
}
