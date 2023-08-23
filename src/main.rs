use crossterm::{cursor, execute, style, terminal};

use tui_bricks::error::Result;
use tui_bricks::state::State;

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: std::io::Write,
{
    let mut state = State::new()?;

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
