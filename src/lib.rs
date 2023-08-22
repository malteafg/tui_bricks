use crossterm::{cursor, execute, style, terminal};

mod command;
mod data;
mod display;
pub mod error;
pub mod io;
mod mode;
mod state;

use error::Result;
use state::State;

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: std::io::Write,
{
    let mut state = State::new()?;

    execute!(w, terminal::EnterAlternateScreen)?;

    loop {
        if state.accept_cmd(w)? {
            break;
        }
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    Ok(())
}
