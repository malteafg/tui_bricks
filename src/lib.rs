use crossterm::{
    cursor, execute, queue,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};

mod command;
mod data;
mod display;
pub mod error;
pub mod io;
mod state;

use data::Item;
use display::EmitMode;
use error::Result;
use state::{Mode, State};

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: std::io::Write,
{
    let mut state = State::new()?;

    execute!(w, terminal::EnterAlternateScreen)?;

    loop {
        display::clear(w)?;
        display::header(w)?;

        let curr_mode = state.get_mode();
        curr_mode.emit_mode(w)?;
        let possible_cmds = curr_mode.get_possible_cmds();
        display::emit_iter(w, possible_cmds.iter())?;
        w.flush()?;

        match io::wait_for_char()? {
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape)?;
                break;
            }
            c => {
                let new_mode = execute_cmd(c, w, &mut state)?;
                state.set_mode(new_mode);
            }
        };
    }

    execute!(w, ResetColor, cursor::Show, terminal::LeaveAlternateScreen)?;

    Ok(())
}

fn execute_cmd<W: std::io::Write>(char: char, w: &mut W, state: &mut State) -> Result<Mode> {
    match char {
        'a' => add_part(w, state),
        'p' => search_part(w, state),
        _ => Ok(Mode::Default {
            info: "executing command failed".to_owned(),
        }),
    }
}

fn add_part<W: std::io::Write>(w: &mut W, state: &mut State) -> Result<Mode> {
    queue!(
        w,
        ResetColor,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print("Enter ID of new part"),
        cursor::MoveToNextLine(1),
        cursor::Show,
    )?;
    w.flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let part_id: u32 = input.trim().parse()?;

    queue!(
        w,
        ResetColor,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print("Select a color group by typing its first letter"),
        cursor::MoveToNextLine(1),
        Print("(you can add more later)"),
        cursor::MoveToNextLine(1),
        cursor::Hide,
    )?;
    w.flush()?;

    for (i, color) in data::COMP_COLORS.iter().enumerate() {
        queue!(
            w,
            Print(format!("{}: {}", i, color)),
            cursor::MoveToNextLine(1),
        )?;
    }
    w.flush()?;

    use data::ColorGroup::*;
    let color_group = match io::wait_for_char()? {
        'a' => All,
        'b' => Basic,
        'n' => Nature,
        'g' => Grey,
        'r' => Road,
        't' => Translucent,
        _ => todo!(),
    };

    queue!(
        w,
        ResetColor,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print(format!("Enter location of group {}:", color_group)),
        cursor::MoveToNextLine(1),
        cursor::Show,
    )?;
    w.flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let part_loc = input.trim();

    let new_item = Item::new(part_id, color_group, part_loc.to_owned());

    state.add_item(new_item.clone())?;

    Ok(Mode::DisplayItem { item: new_item })
}

fn search_part<W>(w: &mut W, state: &State) -> Result<Mode>
where
    W: std::io::Write,
{
    queue!(
        w,
        ResetColor,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        Print("Search for part by ID:"),
        cursor::MoveToNextLine(1),
        cursor::Show,
    )?;
    w.flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let searched_id: u32 = input.trim().parse()?;

    if let Some(item) = state.get_item(searched_id) {
        return Ok(Mode::DisplayItem { item: item.clone() });
    }

    Ok(Mode::Default {
        info: format!("Part {} not found in database", searched_id),
    })
}
