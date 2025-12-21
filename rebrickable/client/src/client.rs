use rebrickable_database_api::RebrickableDB;
use rebrickable_server_api::query::{
    ColorFindType, ColorGetType, FindItem, GetItem, PartFindType, PartGetType, Query,
};
use utils::PathExt;

use std::fmt::Display;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn write_iter(mut writer: impl Write, iter: impl Iterator<Item = impl Display>) {
    for key in iter {
        match writeln!(writer, "{}", key) {
            Ok(_) => {
                writer.flush().unwrap();
            }
            Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                eprintln!("fzf exited early (Broken pipe), stopping write loop");
                break;
            }
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}

pub fn run_fzf<D: RebrickableDB>(database: &D, find_item: FindItem) {
    let dst_path = PathBuf::cache_dir().join("displayed_image.png");
    let images_path = PathBuf::data_dir().join("part_images");
    let sub_cmd = match find_item {
        FindItem::Part {
            part: PartFindType::Id,
        } => "part id",
        FindItem::Part {
            part: PartFindType::Name,
        } => "part name",
        FindItem::Color {
            color: ColorFindType::Id,
        } => "color id",
        FindItem::Color {
            color: ColorFindType::Name,
        } => "color name",
        FindItem::Element => "element",
    };

    let update_image_cmd = format!(
        "tui_bricks_update_image --dst-path=\"{}\" --images-path=\"{}\" {} {{}}",
        dst_path.display(),
        images_path.display(),
        sub_cmd,
    );

    let mut child = Command::new("fzf")
        // .arg("--bind=focus:execute(sh -c '[ -f ../raw_data/parts_red/{}.png ] && cp ../raw_data/parts_red/{}.png ../raw_data/test_image.png' sh {})")
        // .arg(&format!(
        //     "--bind=focus:execute({} &>/dev/null &)",
        //     update_image_cmd
        // ))
        // .arg("--preview=(echo {})")
        .arg(format!("--preview=({})", update_image_cmd))
        .arg("--preview-window=up:30%:wrap")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin").unwrap();

        match find_item {
            FindItem::Part {
                part: PartFindType::Id,
            } => write_iter(stdin, database.iter_part_id()),
            FindItem::Part {
                part: PartFindType::Name,
            } => write_iter(stdin, database.iter_part_name()),
            FindItem::Color {
                color: ColorFindType::Id,
            } => write_iter(stdin, database.iter_color_id()),
            FindItem::Color {
                color: ColorFindType::Name,
            } => write_iter(stdin, database.iter_color_name()),
            FindItem::Element => write_iter(stdin, database.iter_element_id()),
        };
    }

    let output = child.wait_with_output().unwrap();
    let selected_key = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selected_key.is_empty() {
        println!("No key selected");
        return;
    }

    match find_item {
        FindItem::Part {
            part: PartFindType::Id,
        } => match database.part_from_id(&selected_key.into()) {
            Some(part) => println!("{}", part),
            None => println!("Could not find part"),
        },
        FindItem::Part {
            part: PartFindType::Name,
        } => match database.part_from_name(&selected_key.into()) {
            Some(part) => println!("{}", part),
            None => println!("Could not find part"),
        },
        FindItem::Color {
            color: ColorFindType::Id,
        } => match database.color_from_id(&selected_key.parse().unwrap()) {
            Some(color) => println!("{}", color),
            None => println!("Could not find color"),
        },
        FindItem::Color {
            color: ColorFindType::Name,
        } => match database.color_from_name(&selected_key.into()) {
            Some(color) => println!("{}", color),
            None => println!("Could not find color"),
        },
        FindItem::Element => match database.element_from_id(&selected_key.parse().unwrap()) {
            Some(element) => println!("{}", element),
            None => println!("Could not find element"),
        },
    };
}

pub fn handle_query<D: RebrickableDB>(database: &D, query: Query) {
    match query {
        Query::Get { get_item } => match get_item {
            GetItem::Part { part } => match part {
                PartGetType::Id { id } => match database.part_from_id(&id) {
                    Some(part) => println!("{}", part),
                    None => println!("Could not find part with id {}", id),
                },
                PartGetType::Name { name } => match database.part_from_name(&name) {
                    Some(part) => println!("{}", part),
                    None => println!("Could not find part with name {}", name),
                },
            },
            GetItem::Color { color } => match color {
                ColorGetType::Id { id } => match database.color_from_id(&id) {
                    Some(color) => println!("{}", color),
                    None => println!("Could not find color with id {}", id),
                },
                ColorGetType::Name { name } => match database.color_from_name(&name) {
                    Some(color) => println!("{}", color),
                    None => println!("Could not find color with name {}", name),
                },
            },
            GetItem::Element { id } => match database.element_from_id(&id) {
                Some(element) => println!("{}", element),
                None => println!("Could not find element with id {}", id),
            },
        },
        Query::Find {
            find_item: item_type,
        } => {
            run_fzf(database, item_type);
        }
    };
}
