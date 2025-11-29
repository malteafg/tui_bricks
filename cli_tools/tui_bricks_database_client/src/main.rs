use database::{GetItemQuery, Query, Response};

use std::io::{Read, Write};
use std::net::TcpStream;

use bincode;

fn send_query(stream: &mut TcpStream, q: Query) -> Response {
    let data = bincode::encode_to_vec(&q, bincode::config::standard()).unwrap();
    let len = (data.len() as u32).to_le_bytes();

    stream.write_all(&len).unwrap();
    stream.write_all(&data).unwrap();

    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();
    let msg_len = u32::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0u8; msg_len];
    stream.read_exact(&mut buf).unwrap();

    let (response, _): (Response, usize) =
        bincode::decode_from_slice(&buf, bincode::config::standard()).unwrap();

    response
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4000")?;
    println!("Connected to server.");

    let r = send_query(
        &mut stream,
        Query::GetItem(GetItemQuery::PartFromId("4070".into())),
    );
    println!("Response: {:?}", r);

    let r = send_query(
        &mut stream,
        Query::GetItem(GetItemQuery::ColorFromName("Blue".into())),
    );
    println!("Response: {:?}", r);

    Ok(())
}
