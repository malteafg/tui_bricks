use std::time::Duration;

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
use error::Result;

const MENU: &str = r#"TUI Bricks

Controls:
 - 'a' - add a part
 - 'p' - search for a part
 - 'q' - quit program
"#;

pub fn run<W>(w: &mut W, mut database: Database) -> Result<()>
where
    W: std::io::Write,
{
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
            '1' => {
                execute!(w, Print("eyo\n\n"), cursor::MoveToNextLine(1))?;
                // w.flush()?;
                std::thread::sleep(Duration::from_secs(1));

                // execute!(
                //     w,
                //     SetForegroundColor(Color::Blue),
                //     SetBackgroundColor(Color::Red),
                //     Print("Styled text here."),
                //     ResetColor
                // )?;
            }
            'a' => {
                if let Some(part) = add_part(w, &mut database) {
                    search_result = part;
                }
            }
            'p' => {
                search_result = search_part(w, &database);
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

fn add_part<W>(w: &mut W, database: &mut Database) -> Option<String>
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
    )
    .unwrap();
    w.flush().unwrap();

    terminal::disable_raw_mode().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let part_id: u32 = input.trim().parse().unwrap();

    terminal::enable_raw_mode().unwrap();

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
    )
    .unwrap();
    w.flush().unwrap();

    for (i, color) in data::COMP_COLORS.iter().enumerate() {
        queue!(
            w,
            Print(format!("{}: {}", i, color)),
            cursor::MoveToNextLine(1),
        )
        .unwrap();
    }
    w.flush().unwrap();

    use data::ColorGroup::*;
    let color_group = match read_char().unwrap() {
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
    )
    .unwrap();
    w.flush().unwrap();

    terminal::disable_raw_mode().unwrap();

    let mut part_loc = String::new();
    std::io::stdin().read_line(&mut part_loc).unwrap();

    terminal::enable_raw_mode().unwrap();

    let new_item = Item::new(part_id, color_group, part_loc);
    let item_data = new_item.to_string();

    database.push(new_item);

    Some(item_data)
}

fn search_part<W>(w: &mut W, database: &Database) -> String
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
    )
    .unwrap();
    w.flush().unwrap();

    terminal::disable_raw_mode().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let searched_id: u32 = input.trim().parse().unwrap();
    for item in database.iter() {
        if item.get_id() == searched_id {
            terminal::enable_raw_mode().unwrap();
            return item.to_string();
        }
    }

    terminal::enable_raw_mode().unwrap();
    String::new()
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
