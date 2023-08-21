use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};

mod data;
pub mod error;
pub mod io;

use data::{Database, Item};
use error::{Error, Result};

const MENU: &str = r#"TUI Bricks

Controls:
 - 'a' - add a part
 - 'p' - search for a part
 - 'q' - quit program
"#;

pub fn run<W>(w: &mut W) -> Result<()>
where
    W: std::io::Write,
{
    let database_path = io::get_default_database_path()?;
    let mut database = io::read_database_from_path(&database_path)?;

    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut search_result = String::new();

    loop {
        queue!(
            w,
            ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        for line in MENU.split('\n') {
            queue!(w, Print(line), cursor::MoveToNextLine(1))?;
        }
        for line in search_result.split('\n') {
            queue!(w, Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        match read_char()? {
            'a' => {
                if let Ok(part) = add_part(w, &mut database) {
                    search_result = part;
                    io::write_database_to_path(&database_path, &database)?;
                }
            }
            'p' => {
                let result = search_part(w, &database);
                if let Err(Error::PartNotFound { part_id }) = result {
                    search_result = format!("Part {} does not exist in database.", part_id);
                } else {
                    search_result = result?;
                }
            }
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape)?;
                break;
            }
            _ => {}
        };
    }

    execute!(w, ResetColor, cursor::Show, terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    Ok(())
}

fn add_part<W>(w: &mut W, database: &mut Database) -> Result<String>
where
    W: std::io::Write,
{
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

    terminal::disable_raw_mode()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let part_id: u32 = input.trim().parse()?;

    terminal::enable_raw_mode()?;

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
    let color_group = match read_char()? {
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

    terminal::disable_raw_mode()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let part_loc = input.trim();

    terminal::enable_raw_mode()?;

    let new_item = Item::new(part_id, color_group, part_loc.to_owned());
    let item_data = new_item.to_string();

    database.push(new_item);

    Ok(item_data)
}

fn search_part<W>(w: &mut W, database: &Database) -> Result<String>
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

    terminal::disable_raw_mode()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let searched_id: u32 = input.trim().parse()?;
    for item in database.iter() {
        if item.get_id() == searched_id {
            terminal::enable_raw_mode()?;
            return Ok(item.to_string());
        }
    }

    terminal::enable_raw_mode()?;

    Err(Error::PartNotFound {
        part_id: searched_id,
    })
}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
