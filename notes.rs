// server

use bytes::BytesMut;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

// Example request/response types (you can add any you want)
#[derive(Serialize, Deserialize, Debug)]
enum Request {
    GetItem { key: String },
    SetItem { key: String, value: String },
}

#[derive(Serialize, Deserialize, Debug)]
enum Response {
    ItemValue(Option<String>),
    Ok,
    Error(String),
}

// Minimal key-value store for demonstration
use std::sync::{Arc, Mutex};
type Db = Arc<Mutex<std::collections::HashMap<String, String>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db: Db = Arc::new(Mutex::new(std::collections::HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("DB server running on 127.0.0.1:9000");

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            let mut framed = Framed::new(socket, LengthDelimitedCodec::new());

            while let Some(Ok(bytes)) = framed.next().await {
                let req: Request = bincode::deserialize(&bytes).unwrap();

                let resp = handle_request(req, &db);

                let resp_bytes = bincode::serialize(&resp).unwrap();
                framed.send(resp_bytes.into()).await.unwrap();
            }
        });
    }
}

fn handle_request(req: Request, db: &Db) -> Response {
    match req {
        Request::SetItem { key, value } => {
            db.lock().unwrap().insert(key, value);
            Response::Ok
        }
        Request::GetItem { key } => {
            let result = db.lock().unwrap().get(&key).cloned();
            Response::ItemValue(result)
        }
    }
}

// async client
use bytes::BytesMut;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[derive(Serialize, Deserialize, Debug)]
enum Request {
    GetItem { key: String },
    SetItem { key: String, value: String },
}

#[derive(Serialize, Deserialize, Debug)]
enum Response {
    ItemValue(Option<String>),
    Ok,
    Error(String),
}

pub async fn send_request(req: Request) -> anyhow::Result<Response> {
    let stream = TcpStream::connect("127.0.0.1:9000").await?;
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    let bytes = bincode::serialize(&req)?;
    framed.send(bytes.into()).await?;

    let reply = framed.next().await.unwrap()?;
    let response: Response = bincode::deserialize(&reply)?;

    Ok(response)
}
// Example client code:
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = send_request(Request::SetItem {
        key: "foo".into(),
        value: "bar".into(),
    })
    .await?;

    println!("Response: {:?}", res);

    let res = send_request(Request::GetItem { key: "foo".into() }).await?;

    println!("Response: {:?}", res);

    Ok(())
}

// blocking client
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetItem { key: String },
    SetItem { key: String, value: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    ItemValue(Option<String>),
    Ok,
    Error(String),
}

pub fn send_request(req: Request) -> anyhow::Result<Response> {
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;

    // --- serialize request ---
    let bytes = bincode::serialize(&req)?;
    let len = (bytes.len() as u32).to_be_bytes();

    // --- send length prefix + message ---
    stream.write_all(&len)?;
    stream.write_all(&bytes)?;

    // --- read length prefix of the response ---
    let mut len_buf = [0u8; 4];
    stream.read_exact(&len_buf)?;
    let msg_len = u32::from_be_bytes(len_buf) as usize;

    // --- read message ---
    let mut msg_buf = vec![0u8; msg_len];
    stream.read_exact(&msg_buf)?;

    let resp: Response = bincode::deserialize(&msg_buf)?;
    Ok(resp)
}

// Example blocking client usage
fn main() -> anyhow::Result<()> {
    let resp = send_request(Request::SetItem {
        key: "foo".into(),
        value: "bar".into(),
    })?;

    println!("Response: {:?}", resp);

    let resp = send_request(Request::GetItem { key: "foo".into() })?;

    println!("Response: {:?}", resp);

    Ok(())
}
