use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicBool};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use rebrickable_database::LocalDB;
use rebrickable_database_api::RebrickableDB;
use rebrickable_server_api::query::{
    ColorFindType, ColorGetType, FindItem, GetItem, PartFindType, PartGetType, Query,
};
use rebrickable_server_api::response::{GetItemResponse, IterItemsResponse, Response};
use utils::TcpExt;

struct ClientHandler<D: RebrickableDB> {
    stream: TcpStream,
    running: Arc<AtomicBool>,
    database: Arc<D>,
}

impl<D: RebrickableDB> ClientHandler<D> {
    pub fn new(stream: TcpStream, running: Arc<AtomicBool>, database: Arc<D>) -> Self {
        Self {
            stream,
            running,
            database,
        }
    }

    /// Listens for queries sent from the client and sends a response back
    pub fn listen(&mut self) {
        while self.running.load(Ordering::SeqCst) {
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

    fn handle_query(&mut self, query: Query) -> Result<(), Error> {
        match query {
            Query::Get { get_item } => {
                let response = match &get_item {
                    GetItem::Part { part } => match part {
                        PartGetType::Id { id } => match self.database.part_from_id(id) {
                            Some(part) => GetItemResponse::Part(part.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                        PartGetType::Name { name } => match self.database.part_from_name(name) {
                            Some(part) => GetItemResponse::Part(part.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                    },
                    GetItem::Color { color } => match color {
                        ColorGetType::Id { id } => match self.database.color_from_id(id) {
                            Some(color) => GetItemResponse::Color(color.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                        ColorGetType::Name { name } => match self.database.color_from_name(name) {
                            Some(color) => GetItemResponse::Color(color.into_owned()),
                            None => GetItemResponse::NotFound,
                        },
                    },
                    GetItem::Element { id } => match self.database.element_from_id(id) {
                        Some(element) => GetItemResponse::Element(element.into_owned()),
                        None => GetItemResponse::NotFound,
                    },
                };
                self.stream.send(Response::GetItem(response, get_item))
            }
            Query::Find {
                find_item: item_type,
            } => {
                match item_type {
                    FindItem::Part { part } => match part {
                        PartFindType::Id => {
                            for id in self.database.iter_part_id() {
                                self.stream.send(Response::IterItems(Some(
                                    IterItemsResponse::PartId(id.into_owned()),
                                )))?;
                            }
                        }
                        PartFindType::Name => {
                            for name in self.database.iter_part_name() {
                                self.stream.send(Response::IterItems(Some(
                                    IterItemsResponse::PartName(name.into_owned()),
                                )))?;
                            }
                        }
                    },
                    FindItem::Color { color } => match color {
                        ColorFindType::Id => {
                            for id in self.database.iter_color_id() {
                                self.stream.send(Response::IterItems(Some(
                                    IterItemsResponse::ColorId(id.into_owned()),
                                )))?;
                            }
                        }
                        ColorFindType::Name => {
                            for name in self.database.iter_color_name() {
                                self.stream.send(Response::IterItems(Some(
                                    IterItemsResponse::ColorName(name.into_owned()),
                                )))?;
                            }
                        }
                    },
                    FindItem::Element => {
                        for id in self.database.iter_element_id() {
                            self.stream.send(Response::IterItems(Some(
                                IterItemsResponse::ElementId(id.into_owned()),
                            )))?;
                        }
                    }
                }
                self.stream.send(Response::IterItems(None))
            }
        }
    }
}

pub struct RebrickableServer {
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl RebrickableServer {
    pub fn start_with_arc(running: Arc<AtomicBool>) -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:4000")?;
        listener.set_nonblocking(true)?;

        let running_main = Arc::clone(&running);

        let handle = Some(thread::spawn(move || {
            let database = Arc::new(LocalDB::default());
            let mut threads = vec![];
            while running_main.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let database_thread = database.clone();
                        let running_thread = running_main.clone();
                        let mut client_handler =
                            ClientHandler::new(stream, running_thread, database_thread);

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
        }));

        Ok(Self { running, handle })
    }

    pub fn start() -> std::io::Result<Self> {
        let running = Arc::new(AtomicBool::new(true));
        Self::start_with_arc(running)
    }

    pub fn shutdown(&mut self) {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
