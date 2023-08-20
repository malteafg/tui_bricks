use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};

extern crate strum;
#[macro_use]
extern crate strum_macros;

mod data;
use data::Database;

fn main() {
    let test = data::get_test_database();

    let string = serde_yaml::to_string(&test).unwrap();
    dbg!(&string);

    let file_path = "output.txt"; // Change this to your desired file path.

    // Attempt to create or open the file for writing.
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating/opening the file: {}", e);
            return;
        }
    };

    // Attempt to write the string to the file.
    match file.write_all(string.as_bytes()) {
        Ok(()) => println!("Successfully wrote to the file."),
        Err(e) => eprintln!("Error writing to the file: {}", e),
    }

    // Attempt to open the file for reading.
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            return;
        }
    };

    // Read the contents of the file into a String.
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("Contents of the file:\n{}", contents),
        Err(e) => eprintln!("Error reading the file: {}", e),
    }

    let yaml = serde_yaml::from_str(&string).unwrap();
    assert_eq!(test, yaml);

    let mut stdout = std::io::stdout();
    run(&mut stdout, test).unwrap();
}

const MENU: &str = r#"TUI Bricks

Controls:
 - 'a' - add a part
 - 'p' - search for a part
 - 'q' - quit program
"#;

fn run<W>(w: &mut W, database: Database) -> std::io::Result<()>
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
                execute!(w, Print("coming soon"), cursor::MoveToNextLine(1))?;
                std::thread::sleep(Duration::from_secs(1));
            }
            'p' => {
                search_result = search_part(w, &database);
            }
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                break;
            }
            _ => {}
        };
    }

    execute!(w, ResetColor, cursor::Show, terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()
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

pub fn read_char() -> std::io::Result<char> {
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
