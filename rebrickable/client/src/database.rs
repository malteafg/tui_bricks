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

    fn send_and_receive(&self, query: Query) -> Result<Response, TcpError> {
        self.stream.borrow_mut().send_and_receive(&query)
    }
}

struct ResponseIter<'a, T> {
    stream: &'a RefCell<TcpStream>,
    _marker: PhantomData<T>,
}

impl<'a, T> ResponseIter<'a, T> {
    fn new(stream: &'a RefCell<TcpStream>) -> Self {
        Self {
            stream,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Iterator for ResponseIter<'a, T> {
    type Item = IterItemsResponse;

    fn next(&mut self) -> Option<Self::Item> {
        let response = self.stream.borrow_mut().receive();
        let Ok(response) = response else {
            return None;
        };

        match response {
            Response::IterItems(iter_response) => iter_response,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }
}

impl RebrickableDB for ClientDB {
    fn part_from_id(&self, id: &PartId) -> Option<Cow<'_, Part>> {
        let query = Query::from(id.clone());
        let response = self.send_and_receive(query);
        let Ok(response) = response else {
            return None;
        };

        match response {
            Response::GetItem(GetItemResponse::Part(part), _) => Some(Cow::Owned(part)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn part_from_name(&self, name: &PartName) -> Option<Cow<'_, Part>> {
        let query = Query::from(name.clone());
        let response = self.send_and_receive(query);
        let Ok(response) = response else {
            return None;
        };

        match response {
            Response::GetItem(GetItemResponse::Part(part), _) => Some(Cow::Owned(part)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn color_from_id(&self, id: &ColorId) -> Option<Cow<'_, Color>> {
        let query = Query::from(*id);
        let response = self.send_and_receive(query);
        let Ok(response) = response else {
            return None;
        };

        match response {
            Response::GetItem(GetItemResponse::Color(color), _) => Some(Cow::Owned(color)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn color_from_name(&self, name: &ColorName) -> Option<Cow<'_, Color>> {
        let query = Query::from(name.clone());
        let response = self.send_and_receive(query);
        let Ok(response) = response else {
            return None;
        };

        match response {
            Response::GetItem(GetItemResponse::Color(color), _) => Some(Cow::Owned(color)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn element_from_id(&self, id: &ElementId) -> Option<Cow<'_, Element>> {
        let query = Query::from(*id);
        let response = self.send_and_receive(query);
        let Ok(response) = response else {
            return None;
        };

        match response {
            Response::GetItem(GetItemResponse::Element(element), _) => Some(Cow::Owned(element)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn iter_part_id(&self) -> impl Iterator<Item = Cow<'_, PartId>> {
        let query = Query::Find(FindItem::PartId);
        self.stream.borrow_mut().send(&query).unwrap();
        let iter = ResponseIter::<IterItemsResponse>::new(&self.stream);

        iter.map(|element| match element {
            IterItemsResponse::PartId(part_id) => Cow::Owned(part_id),
            response_iter => {
                eprintln!("ResponseIter: {:#?}", response_iter);
                panic!();
            }
        })
    }

    fn iter_part_name(&self) -> impl Iterator<Item = Cow<'_, PartName>> {
        let query = Query::Find(FindItem::PartName);
        self.stream.borrow_mut().send(&query).unwrap();
        let iter = ResponseIter::<IterItemsResponse>::new(&self.stream);

        iter.map(|element| match element {
            IterItemsResponse::PartName(part_name) => Cow::Owned(part_name),
            response_iter => {
                eprintln!("ResponseIter: {:#?}", response_iter);
                panic!();
            }
        })
    }

    fn iter_color_id(&self) -> impl Iterator<Item = Cow<'_, ColorId>> {
        let query = Query::Find(FindItem::ColorId);
        self.stream.borrow_mut().send(&query).unwrap();
        let iter = ResponseIter::<IterItemsResponse>::new(&self.stream);

        iter.map(|element| match element {
            IterItemsResponse::ColorId(color_id) => Cow::Owned(color_id),
            response_iter => {
                eprintln!("ResponseIter: {:#?}", response_iter);
                panic!();
            }
        })
    }

    fn iter_color_name(&self) -> impl Iterator<Item = Cow<'_, ColorName>> {
        let query = Query::Find(FindItem::ColorName);
        self.stream.borrow_mut().send(&query).unwrap();
        let iter = ResponseIter::<IterItemsResponse>::new(&self.stream);

        iter.map(|element| match element {
            IterItemsResponse::ColorName(color_name) => Cow::Owned(color_name),
            response_iter => {
                eprintln!("ResponseIter: {:#?}", response_iter);
                panic!();
            }
        })
    }

    fn iter_element_id(&self) -> impl Iterator<Item = Cow<'_, ElementId>> {
        let query = Query::Find(FindItem::Element);
        self.stream.borrow_mut().send(&query).unwrap();
        let iter = ResponseIter::<IterItemsResponse>::new(&self.stream);

        iter.map(|element| match element {
            IterItemsResponse::ElementId(element_id) => Cow::Owned(element_id),
            response_iter => {
                eprintln!("ResponseIter: {:#?}", response_iter);
                panic!();
            }
        })
    }
}
