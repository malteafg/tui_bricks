use crate::rebrickable_database::*;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum GetItemQuery {
    PartFromId(PartId),
    PartFromName(String),
    ColorFromId(ColorId),
    ColorFromName(String),
    ElementFromId(ElementId),
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum IterItemsQuery {
    PartId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Query {
    GetItem(GetItemQuery),
    IterItems(IterItemsQuery),
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum GetItemResponse {
    Part(Part),
    Color(ColorRecord),
    Element(ElementRecord),
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum IterItemsResponse {
    PartId(PartId),
    ColorId(ColorId),
    ElementId(ElementId),
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Response {
    GetItem(GetItemResponse, GetItemQuery),
    /// The Option is None once the stream has ended.
    IterItems(Option<IterItemsResponse>),
}
