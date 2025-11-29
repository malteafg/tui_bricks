use crate::rebrickable_database::*;
use crate::rebrickable_server::*;

use utils::TcpExt;

use std::borrow::Cow;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::net::TcpStream;

pub struct ClientDB {
    stream: RefCell<TcpStream>,
}

impl ClientDB {
    pub fn new() -> Self {
        Self {
            stream: RefCell::new(TcpStream::connect("127.0.0.1:4000").unwrap()),
        }
    }

    fn send_and_receive(&self, query: Query) -> Response {
        self.stream.borrow_mut().send_and_receive(query)
    }
}

struct ResponseIter<'a, T> {
    stream: &'a RefCell<TcpStream>,
    _marker: PhantomData<T>,
}

impl<'a, T> ResponseIter<'a, T> {
    fn new(stream: &'a RefCell<TcpStream>, query: Query) -> Self {
        stream.borrow_mut().send(query);
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

        match response {
            Response::IterItems(iter_response) => iter_response,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }
}

impl<'a> RebrickableDB<'a> for ClientDB {
    fn part_from_id(&self, id: &PartId) -> Option<Cow<'a, Part>> {
        let query = Query::GetItem(GetItemQuery::PartFromId(id.clone()));
        let response = self.send_and_receive(query);

        match response {
            Response::GetItem(GetItemResponse::Part(part), _) => Some(Cow::Owned(part)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn part_from_name(&self, name: &str) -> Option<Cow<'a, Part>> {
        let query = Query::GetItem(GetItemQuery::PartFromName(name.to_string()));
        let response = self.send_and_receive(query);

        match response {
            Response::GetItem(GetItemResponse::Part(part), _) => Some(Cow::Owned(part)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn color_from_id(&self, id: &ColorId) -> Option<Cow<'a, ColorRecord>> {
        let query = Query::GetItem(GetItemQuery::ColorFromId(id.clone()));
        let response = self.send_and_receive(query);

        match response {
            Response::GetItem(GetItemResponse::Color(color), _) => Some(Cow::Owned(color)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn color_from_name(&self, name: &str) -> Option<Cow<'a, ColorRecord>> {
        let query = Query::GetItem(GetItemQuery::ColorFromName(name.to_string()));
        let response = self.send_and_receive(query);

        match response {
            Response::GetItem(GetItemResponse::Color(color), _) => Some(Cow::Owned(color)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn element_from_id(&self, id: &ElementId) -> Option<Cow<'a, ElementRecord>> {
        let query = Query::GetItem(GetItemQuery::ElementFromId(id.clone()));
        let response = self.send_and_receive(query);

        match response {
            Response::GetItem(GetItemResponse::Element(element), _) => Some(Cow::Owned(element)),
            Response::GetItem(GetItemResponse::NotFound, _) => None,
            response => {
                eprintln!("Response: {:#?}", response);
                panic!();
            }
        }
    }

    fn iter_part_id(&self) -> impl Iterator<Item = Cow<'a, PartId>> {
        let query = Query::IterItems(IterItemsQuery::PartId);
        let iter = ResponseIter::<IterItemsResponse>::new(&self.stream, query);

        iter.map(|element| match element {
            IterItemsResponse::PartId(part_id) => Cow::Owned(part_id),
            response_iter => {
                eprintln!("ResponseIter: {:#?}", response_iter);
                panic!();
            }
        })
    }
}
