use rebrickable_database_api::*;

use rebrickable_server_api::query::{FindItem, Query};
use rebrickable_server_api::response::{GetItemResponse, IterItemsResponse, Response};
use utils::{TcpError, TcpExt};

use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Error;
use std::marker::PhantomData;
use std::net::TcpStream;

pub struct ClientDB {
    stream: RefCell<TcpStream>,
}

impl ClientDB {
    pub fn new() -> Result<Self, Error> {
        let stream = TcpStream::connect("127.0.0.1:4000")?;
        Ok(Self {
            stream: RefCell::new(stream),
        })
    }

    fn send_query(&self, query: impl Into<Query>) -> Result<(), TcpError> {
        self.stream.borrow_mut().send(&query.into())
    }

    fn receive_response(&self) -> Result<Response, TcpError> {
        self.stream.borrow_mut().receive()
    }
}

struct ResponseIter<'a, T> {
    stream: Option<&'a RefCell<TcpStream>>,
    _marker: PhantomData<T>,
}

impl<'a, T> ResponseIter<'a, T> {
    fn new() -> Self {
        Self {
            stream: None,
            _marker: std::marker::PhantomData,
        }
    }

    fn with_tcp_stream(stream: &'a RefCell<TcpStream>) -> Self {
        Self {
            stream: Some(stream),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Iterator for ResponseIter<'a, T> {
    type Item = IterItemsResponse;

    fn next(&mut self) -> Option<Self::Item> {
        let response = self.stream?.borrow_mut().receive();
        let Ok(response) = response else {
            return None;
        };

        match response {
            Response::IterItems(iter_response) => iter_response,
            _ => self.next(),
        }
    }
}

impl RebrickableDB for ClientDB {
    fn part_from_id(&self, id: &PartId) -> Option<Cow<'_, Part>> {
        self.send_query(id.clone()).ok()?;
        loop {
            match self.receive_response() {
                Ok(Response::GetItem(GetItemResponse::Part(part), _)) => {
                    return Some(Cow::Owned(part));
                }
                Ok(Response::GetItem(GetItemResponse::NotFound, _)) | Err(_) => return None,
                _ => {}
            }
        }
    }

    fn part_from_name(&self, name: &PartName) -> Option<Cow<'_, Part>> {
        self.send_query(name.clone()).ok()?;
        loop {
            match self.receive_response() {
                Ok(Response::GetItem(GetItemResponse::Part(part), _)) => {
                    return Some(Cow::Owned(part));
                }
                Ok(Response::GetItem(GetItemResponse::NotFound, _)) | Err(_) => return None,
                _ => {}
            }
        }
    }

    fn color_from_id(&self, id: &ColorId) -> Option<Cow<'_, Color>> {
        self.send_query(*id).ok()?;
        loop {
            match self.receive_response() {
                Ok(Response::GetItem(GetItemResponse::Color(color), _)) => {
                    return Some(Cow::Owned(color));
                }
                Ok(Response::GetItem(GetItemResponse::NotFound, _)) | Err(_) => return None,
                _ => {}
            }
        }
    }

    fn color_from_name(&self, name: &ColorName) -> Option<Cow<'_, Color>> {
        self.send_query(name.clone()).ok()?;
        loop {
            match self.receive_response() {
                Ok(Response::GetItem(GetItemResponse::Color(color), _)) => {
                    return Some(Cow::Owned(color));
                }
                Ok(Response::GetItem(GetItemResponse::NotFound, _)) | Err(_) => return None,
                _ => {}
            }
        }
    }

    fn element_from_id(&self, id: &ElementId) -> Option<Cow<'_, Element>> {
        self.send_query(*id).ok()?;
        loop {
            match self.receive_response() {
                Ok(Response::GetItem(GetItemResponse::Element(element), _)) => {
                    return Some(Cow::Owned(element));
                }
                Ok(Response::GetItem(GetItemResponse::NotFound, _)) | Err(_) => return None,
                _ => {}
            }
        }
    }

    fn iter_part_id(&self) -> impl Iterator<Item = Cow<'_, PartId>> {
        let iter = match self.send_query(FindItem::PartId) {
            Ok(()) => ResponseIter::<IterItemsResponse>::with_tcp_stream(&self.stream),
            Err(_) => ResponseIter::<IterItemsResponse>::new(),
        };

        iter.filter_map(|element| match element {
            IterItemsResponse::PartId(part_id) => Some(Cow::Owned(part_id)),
            _ => None,
        })
    }

    fn iter_part_name(&self) -> impl Iterator<Item = Cow<'_, PartName>> {
        let iter = match self.send_query(FindItem::PartName) {
            Ok(()) => ResponseIter::<IterItemsResponse>::with_tcp_stream(&self.stream),
            Err(_) => ResponseIter::<IterItemsResponse>::new(),
        };

        iter.filter_map(|element| match element {
            IterItemsResponse::PartName(part_name) => Some(Cow::Owned(part_name)),
            _ => None,
        })
    }

    fn iter_color_id(&self) -> impl Iterator<Item = Cow<'_, ColorId>> {
        let iter = match self.send_query(FindItem::ColorId) {
            Ok(()) => ResponseIter::<IterItemsResponse>::with_tcp_stream(&self.stream),
            Err(_) => ResponseIter::<IterItemsResponse>::new(),
        };

        iter.filter_map(|element| match element {
            IterItemsResponse::ColorId(color_id) => Some(Cow::Owned(color_id)),
            _ => None,
        })
    }

    fn iter_color_name(&self) -> impl Iterator<Item = Cow<'_, ColorName>> {
        let iter = match self.send_query(FindItem::ColorName) {
            Ok(()) => ResponseIter::<IterItemsResponse>::with_tcp_stream(&self.stream),
            Err(_) => ResponseIter::<IterItemsResponse>::new(),
        };

        iter.filter_map(|element| match element {
            IterItemsResponse::ColorName(color_name) => Some(Cow::Owned(color_name)),
            _ => None,
        })
    }

    fn iter_element_id(&self) -> impl Iterator<Item = Cow<'_, ElementId>> {
        let iter = match self.send_query(FindItem::Element) {
            Ok(()) => ResponseIter::<IterItemsResponse>::with_tcp_stream(&self.stream),
            Err(_) => ResponseIter::<IterItemsResponse>::new(),
        };

        iter.filter_map(|element| match element {
            IterItemsResponse::ElementId(element_id) => Some(Cow::Owned(element_id)),
            _ => None,
        })
    }
}
