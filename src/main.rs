use std::path::PathBuf;

use crossterm::{cursor, execute, style, terminal};

use tui_bricks::error::Result;
use tui_bricks::state::State;

#[cfg(not(debug_assertions))]
fn get_user_db_path() -> Result<PathBuf> {
    let mut config_path = tui_bricks::io::get_config_dir()?;
    config_path.push("config.yml");
    let config = match tui_bricks::io::read_contents_from_path(&config_path) {
        Ok(config) => config,
        Err(tui_bricks::error::Error::IOError(io_error))
            if io_error.kind() == std::io::ErrorKind::NotFound =>
        {
            let config = tui_bricks::config::Config::new()?;
            tui_bricks::io::write_contents_to_path(&config_path, &config)?;
            config
        }
        Err(e) => return Err(e),
    };
    Ok(config.get_db_path())
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
        if state.accept_cmd(w)? {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    Ok(())
}

fn main() -> tui_bricks::error::Result<()> {
    env_logger::init();
    let mut stdout = std::io::stdout();
    run(&mut stdout)
}
