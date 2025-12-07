mod client;
mod database;

pub use database::ClientDB;

use rebrickable_database::LocalDB;
use rebrickable_server_api::query::Query;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    pub query: Query,
}

pub fn run(args: Args) {
    match ClientDB::new() {
        Ok(database) => client::handle_query(&database, args.query),
        Err(_) => {
            let database = LocalDB::default();
            client::handle_query(&database, args.query);
        }
    }
}
