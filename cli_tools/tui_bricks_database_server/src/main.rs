use database::{Database, DatabaseI, GetItemQuery, GetItemResponse, Query, Response};
use utils;

use bincode;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream, database: &Database) {
    // --- Read length prefix (u32) ---
    let mut len_buf = [0u8; 4];
    if stream.read_exact(&mut len_buf).is_err() {
        eprintln!("Connection closed before reading length");
        return;
    }
    let msg_len = u32::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0u8; msg_len];
    if stream.read_exact(&mut buf).is_err() {
        eprintln!("Failed to read message");
        return;
    }

    let (query, _): (Query, usize) =
        match bincode::decode_from_slice(&buf, bincode::config::standard()) {
            Ok(q) => q,
            Err(e) => {
                eprintln!("bad query: {e}");
                return;
            }
        };

    println!("Received query: {:?}", query);

    let response = match query {
        Query::GetItem(query) => {
            let response = match &query {
                GetItemQuery::PartFromId(id) => match database.part_from_id(&id) {
                    Some(part) => GetItemResponse::Part(part.clone()),
                    None => GetItemResponse::NotFound,
                },
                GetItemQuery::PartFromName(name) => match database.part_from_name(&name) {
                    Some(part) => GetItemResponse::Part(part.clone()),
                    None => GetItemResponse::NotFound,
                },
                GetItemQuery::ColorFromId(id) => match database.color_from_id(&id) {
                    Some(color) => GetItemResponse::Color(color.clone()),
                    None => GetItemResponse::NotFound,
                },
                GetItemQuery::ColorFromName(name) => match database.color_from_name(&name) {
                    Some(color) => GetItemResponse::Color(color.clone()),
                    None => GetItemResponse::NotFound,
                },
            };
            Response::GetItem(response, query);
        }
    };

    let out = bincode::encode_to_vec(&response, bincode::config::standard()).unwrap();
    let len = (out.len() as u32).to_le_bytes();

    let _ = stream.write_all(&len);
    let _ = stream.write_all(&out);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    println!("Server listening on 127.0.0.1:4000");

    let mut parts_path = utils::data_dir();
    parts_path.push("parts.csv");
    dbg!(&parts_path.display());

    let mut colors_path = utils::data_dir();
    colors_path.push("colors.csv");

    let mut elements_path = utils::data_dir();
    elements_path.push("elements.csv");

    let database = Database::new(&parts_path, &colors_path, &elements_path);

    loop {
        let (stream, addr) = listener.accept()?;
        println!("Client connected: {addr}");

        handle_client(stream, &database);
    }
}
