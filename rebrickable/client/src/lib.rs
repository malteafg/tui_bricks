pub mod cli;
mod client;
mod database;

use cli::{ColorFindType, ColorGetType, FindItem, GetItem, PartFindType, PartGetType, Query};
pub use database::ClientDB;

use rebrickable_database::LocalDB;
use rebrickable_server_api::query;
use utils::PathExt;

use std::{path::PathBuf, process::Command};

pub fn run(args: cli::Args) {
    let query = match args.query {
        Query::Get { get_item } => query::Query::Get(match get_item {
            GetItem::Part { part } => match part {
                PartGetType::Id { id } => query::GetItem::PartFromId(id),
                PartGetType::Name { name } => query::GetItem::PartFromName(name),
            },
            GetItem::Color { color } => match color {
                ColorGetType::Id { id } => query::GetItem::ColorFromId(id),
                ColorGetType::Name { name } => query::GetItem::ColorFromName(name),
            },
            GetItem::Element { id } => query::GetItem::Element(id),
        }),
        Query::Find { find_item } => query::Query::Find(match find_item {
            FindItem::Part { part } => match part {
                PartFindType::Id => query::FindItem::PartId,
                PartFindType::Name => query::FindItem::PartName,
            },
            FindItem::Color { color } => match color {
                ColorFindType::Id => query::FindItem::ColorId,
                ColorFindType::Name => query::FindItem::ColorName,
            },
            FindItem::Element => query::FindItem::Element,
        }),
    };

    let mut sxiv_path = PathBuf::cache_dir();
    sxiv_path.push("displayed_image.png");
    let _ = Command::new("sxiv").arg(sxiv_path).spawn().unwrap();

    match ClientDB::new() {
        Ok(database) => client::handle_query(&database, query),
        Err(_) => {
            let database = LocalDB::default();
            client::handle_query(&database, query);
        }
    }
}
