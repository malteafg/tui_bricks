use std::fmt;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::str::FromStr;
use std::time::Duration;

use crossterm::{
    cursor::{self, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
    Command,
};

use serde::{Deserialize, Serialize};

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(EnumString, Display, Serialize, Deserialize, Debug, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum ColorGroup {
    AllColors,
    OtherColorGroup(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Item {
    id: u32,
    alternative_ids: Vec<u32>,
    name: String,
    location: Vec<(ColorGroup, String)>,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut loc_string = "Location of each color group:\n".to_owned();
        for (color_group, loc) in self.location.iter() {
            loc_string.push_str(&format!("{}: {}\n", color_group.to_string(), loc));
        }

        write!(
            f,
            "Part found: {}\n\nPart ID: {}\n\n{}",
            self.name.clone(),
            self.id,
            loc_string,
        )
    }
}

// impl Item {
//     fn print<W>(&self, w: &mut W)
//     where
//         W: std::io::Write,
//     {
//         let name = self.name.clone();
//         let id = self.id;
//         execute!(
//             w,
//             Print("Part found!"),
//             MoveToNextLine(1),
//             Print(format!("{}:", self.name.clone())),
//             MoveToNextLine(1),
//             Print(format!("Part ID: {}", self.id)),
//             MoveToNextLine(1),
//             Print("Location:"),
//             MoveToNextLine(1),
//         )
//         .unwrap();
//         for (color_group, loc) in self.location.iter() {
//             execute!(
//                 w,
//                 Print(format!("{}: {}", color_group.to_string(), loc)),
//                 MoveToNextLine(1)
//             )
//             .unwrap();
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Database {
    items: Vec<Item>,
}

fn main() {
    let item1 = Item {
        id: 44,
        alternative_ids: vec![123, 1324],
        name: String::from_str("Testid").unwrap(),
        location: vec![(ColorGroup::AllColors, String::from_str("B1A3").unwrap())],
    };

    let item2 = Item {
        id: 43,
        alternative_ids: vec![12, 14],
        name: String::from_str("blah blah").unwrap(),
        location: vec![(ColorGroup::AllColors, String::from_str("B1A4").unwrap())],
    };

    let test = Database {
        items: vec![item1, item2],
    };

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

const MENU: &str = r#"
TUI Bricks

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
            cursor::MoveTo(1, 1)
        )?;

        for line in MENU.split('\n') {
            queue!(w, Print(line), cursor::MoveToNextLine(1))?;
        }
        for line in search_result.split('\n') {
            queue!(w, Print(line), cursor::MoveToNextLine(1))?;
        }
        // execute!(w, Print(search_result.clone())).unwrap();

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
            // '2' => test::color::run(w)?,
            // '3' => test::attribute::run(w)?,
            // '4' => test::event::run(w)?,
            // '5' => test::synchronized_output::run(w)?,
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
    terminal::disable_raw_mode().unwrap();

    println!("Search for part");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let searched_id: u32 = input.trim().parse().unwrap();
    for item in database.items.iter() {
        if item.id == searched_id {
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

pub fn buffer_size() -> std::io::Result<(u16, u16)> {
    terminal::size()
}
