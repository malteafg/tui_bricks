use rebrickable_database_api::{PartId, RebrickableDB};
use rebrickable_server_api::query::{ColorGetType, GetItem, ItemType, PartGetType, Query};
use utils::PathExt;

use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn run_fzf<D: RebrickableDB>(database: &D, item_type: ItemType) {
    let dst_path = PathBuf::cache_dir().join("displayed_image.png");
    let images_path = PathBuf::data_dir().join("part_images");
    let update_image_cmd = format!(
        "tui_bricks_update_image {{}} --dst-path=\"{}\" --images-path=\"{}\"",
        dst_path.display(),
        images_path.display()
    );

    let mut child = Command::new("fzf")
        // .arg("--bind=focus:execute(sh -c '[ -f ../raw_data/parts_red/{}.png ] && cp ../raw_data/parts_red/{}.png ../raw_data/test_image.png' sh {})")
        // .arg(&format!(
        //     "--bind=focus:execute({} &>/dev/null &)",
        //     update_image_cmd
        // ))
        // .arg("--preview=(echo {})")
        .arg(&format!("--preview=({})", update_image_cmd))
        .arg("--preview-window=up:30%:wrap")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin").unwrap();

        let iter = match item_type {
            ItemType::Part => database.iter_part_id(),
            ItemType::Color => unimplemented!(),
            ItemType::Element => unimplemented!(),
        };
        for key in iter {
            match writeln!(stdin, "{}", key) {
                Ok(_) => {
                    stdin.flush().unwrap();
                }
                Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                    eprintln!("fzf exited early (Broken pipe), stopping write loop");
                    break;
                }
                Err(e) => return Err(Box::new(e)).unwrap(),
            }
        }
    }

    let output = child.wait_with_output().unwrap();
    let selected_key: PartId = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
        .into();

    match database.part_from_id(&selected_key) {
        Some(part) => println!("{}", part),
        None => println!("Could not find part"),
    }
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
        Query::Find { item_type } => {
            run_fzf(database, item_type);
        }
    };
}
