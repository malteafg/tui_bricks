use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicBool};
use std::thread;
use std::time::Duration;

use rebrickable_database::LocalDB;
use rebrickable_database_api::RebrickableDB;
use rebrickable_server_api::query::{ColorGetType, GetItem, ItemType, PartGetType, Query};
use rebrickable_server_api::response::{GetItemResponse, IterItemsResponse, Response};
use utils::{PathExt, TcpExt};

struct ClientHandler<D> {
    stream: TcpStream,
    shutdown: Arc<AtomicBool>,
    database: Arc<D>,
}

impl<D> ClientHandler<D> {
    pub fn new(stream: TcpStream, shutdown: Arc<AtomicBool>, database: Arc<D>) -> Self {
        Self {
            stream,
            shutdown,
            database,
        }
    }

    /// Listens for queries sent from the client and sends a response back
    pub fn listen(&mut self)
    where
        D: RebrickableDB,
    {
        while !self.shutdown.load(Ordering::SeqCst) {
            match self.stream.receive() {
                Ok(query) => match self.handle_query(query) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Terminating connection. {}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Terminating connection. {}", e);
                    return;
                }
            }
        }
    }

    fn handle_query(&mut self, query: Query) -> Result<(), Error>
    where
        D: RebrickableDB,
    {
        match query {
            Query::Get { get_item } => {
                let response = match &get_item {
                    GetItem::Part { part } => match part {
                        PartGetType::Id { id } => match self.database.part_from_id(&id) {
                            Some(part) => GetItemResponse::Part(part.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                        PartGetType::Name { name } => match self.database.part_from_name(&name) {
                            Some(part) => GetItemResponse::Part(part.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                    },
                    GetItem::Color { color } => match color {
                        ColorGetType::Id { id } => match self.database.color_from_id(&id) {
                            Some(color) => GetItemResponse::Color(color.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                        ColorGetType::Name { name } => match self.database.color_from_name(&name) {
                            Some(color) => GetItemResponse::Color(color.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                    },
                    GetItem::Element { id } => match self.database.element_from_id(&id) {
                        Some(element) => GetItemResponse::Element(element.into_owned()),
                        None => GetItemResponse::NotFound,
                    },
                };
                self.stream.send(Response::GetItem(response, get_item))
            }
            Query::Find { item_type } => {
                match item_type {
                    ItemType::Part => {
                        for id in self.database.iter_part_id() {
                            self.stream.send(Response::IterItems(Some(
                                IterItemsResponse::PartId(id.into_owned()),
                            )))?;
                        }
                    }
                    ItemType::Color => unimplemented!(),
                    ItemType::Element => unimplemented!(),
                }
                self.stream.send(Response::IterItems(None))
            }
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    listener.set_nonblocking(true)?;

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_handler = shutdown.clone();

    ctrlc::set_handler(move || {
        println!("\nCtrl+C received, shutting down…");
        shutdown_handler.store(true, Ordering::SeqCst);
    })
    .unwrap();

    let mut parts_path = PathBuf::data_dir();
    parts_path.push("parts.csv");

    let mut colors_path = PathBuf::data_dir();
    colors_path.push("colors.csv");

    let mut elements_path = PathBuf::data_dir();
    elements_path.push("elements.csv");

    let mut relationships_path = PathBuf::data_dir();
    relationships_path.push("part_relationships.csv");

    let database = Arc::new(LocalDB::new(
        &parts_path,
        &colors_path,
        &elements_path,
        &relationships_path,
    ));

    let mut threads = vec![];

    while !shutdown.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                let database = database.clone();
                let shutdown = shutdown.clone();
                let mut client_handler = ClientHandler::new(stream, shutdown, database);

                threads.push(thread::spawn(move || {
                    client_handler.listen();
                }));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => eprintln!("accept failed: {}", e),
        }
    }

    println!("Waiting for threads...");
    for t in threads {
        let _ = t.join();
    }

    println!("Server exited cleanly.");
    Ok(())
}
