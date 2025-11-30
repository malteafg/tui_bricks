mod database;

pub use database::ClientDB;

use rebrickable_database_api::{PartId, RebrickableDB};
use rebrickable_server_api::query::{ColorGetType, GetItem, ItemType, PartGetType, Query};
use utils::PathExt;

use clap::Parser;

use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    pub query: Query,
}

fn run_fzf<D: RebrickableDB>(database: &D, item_type: ItemType) {
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
        .arg(&format!("--preview=({} && echo {{}})", update_image_cmd))
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

    if let Some(value) = database.part_from_id(&selected_key) {
        println!("You selected: {} => {:#?}", selected_key, value);
    } else {
        println!("Key not found: {}", selected_key);
    };
}

pub fn run(args: Args) {
    let database = ClientDB::new();

    match args.query {
        Query::Get { get_item } => match get_item {
            GetItem::Part { part } => match part {
                PartGetType::Id { id } => {
                    let item = database.part_from_id(&id);
                    dbg!(item);
                }
                PartGetType::Name { name } => {
                    let item = database.part_from_name(&name);
                    dbg!(item);
                }
            },
            GetItem::Color { color } => match color {
                ColorGetType::Id { id } => {
                    let item = database.color_from_id(&id);
                    dbg!(item);
                }
                ColorGetType::Name { name } => {
                    let item = database.color_from_name(&name);
                    dbg!(item);
                }
            },
            GetItem::Element { id } => {
                let item = database.element_from_id(&id);
                dbg!(item);
            }
        },
        Query::Find { item_type } => {
            run_fzf(&database, item_type);
        }
    };
}
