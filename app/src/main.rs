use std::path::PathBuf;

use crossterm::{cursor, execute, style, terminal};

use tui_bricks::error::{Error, Result};
use tui_bricks::state::State;

#[cfg(not(debug_assertions))]
fn get_user_db_path() -> Result<PathBuf> {
    use figment::{
        providers::{Format, Serialized, Yaml},
        Figment,
    };
    use tui_bricks::config::Config;

    let mut config_path = tui_bricks::io::get_config_dir()?;
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

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: std::io::Write,
{
    #[cfg(not(debug_assertions))]
    let db_path = get_user_db_path()?;

    #[cfg(debug_assertions)]
    let db_path = PathBuf::new().join("test_db.yml");

    let mut state = State::new(db_path)?;

    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        match state.wait_for_cmd(w) {
            Ok(()) => {}
            Err(Error::Quit) => {
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

fn main() -> tui_bricks::error::Result<()> {
    env_logger::init();
    let mut stdout = std::io::stdout();
    run(&mut stdout)
}
