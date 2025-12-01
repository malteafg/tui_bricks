mod client;
mod database;

use std::path::PathBuf;

pub use database::ClientDB;

use rebrickable_database::LocalDB;
use rebrickable_server_api::query::Query;

use clap::Parser;
use utils::PathExt;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    pub query: Query,
}

pub fn run(args: Args) {
    match ClientDB::new() {
        Ok(database) => client::handle_query(&database, args.query),
        Err(_) => {
            let mut parts_path = PathBuf::data_dir();
            parts_path.push("parts.csv");

            let mut colors_path = PathBuf::data_dir();
            colors_path.push("colors.csv");

            let mut elements_path = PathBuf::data_dir();
            elements_path.push("elements.csv");

            let database = LocalDB::new(&parts_path, &colors_path, &elements_path);
            client::handle_query(&database, args.query);
        }
    }
}
