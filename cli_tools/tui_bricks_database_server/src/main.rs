use database::rebrickable_server::{GetItemQuery, GetItemResponse, Query, Response};
use database::{LocalDB, RebrickableDB};
use utils::TcpExt;

use std::net::{TcpListener, TcpStream};

fn handle_client<'a, D: RebrickableDB<'a>>(mut stream: TcpStream, database: &'a D) {
    loop {
        let query: Query = stream.receive();

        println!("Received query: {:?}", query);

        let response = match query {
            Query::GetItem(query) => {
                let response = match &query {
                    GetItemQuery::PartFromId(id) => match database.part_from_id(&id) {
                        Some(part) => GetItemResponse::Part(part.into_owned()),
                        None => GetItemResponse::NotFound,
                    },
                    GetItemQuery::PartFromName(name) => match database.part_from_name(&name) {
                        Some(part) => GetItemResponse::Part(part.into_owned()),
                        None => GetItemResponse::NotFound,
                    },
                    GetItemQuery::ColorFromId(id) => match database.color_from_id(&id) {
                        Some(color) => GetItemResponse::Color(color.into_owned()),
                        None => GetItemResponse::NotFound,
                    },
                    GetItemQuery::ColorFromName(name) => match database.color_from_name(&name) {
                        Some(color) => GetItemResponse::Color(color.into_owned()),
                        None => GetItemResponse::NotFound,
                    },
                    GetItemQuery::ElementFromId(id) => match database.element_from_id(&id) {
                        Some(element) => GetItemResponse::Element(element.into_owned()),
                        None => GetItemResponse::NotFound,
                    },
                };
                Response::GetItem(response, query)
            }
            Query::IterItems(_) => unimplemented!(),
        };

        stream.send(response);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    println!("Server listening on 127.0.0.1:4000");

    let mut parts_path = utils::data_dir();
    parts_path.push("parts.csv");

    let mut colors_path = utils::data_dir();
    colors_path.push("colors.csv");

    let mut elements_path = utils::data_dir();
    elements_path.push("elements.csv");

    let database = LocalDB::new(&parts_path, &colors_path, &elements_path);

    loop {
        let (stream, addr) = listener.accept()?;
        println!("Client connected: {addr}");

        handle_client(stream, &database);
    }
}
