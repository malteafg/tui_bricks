use serde::{Deserialize, Serialize};

use std::fmt::Display;

utils::strong_type!(PartId, String);
utils::strong_type!(ColorId, usize, Copy);
utils::strong_type!(ElementId, usize, Copy);
utils::strong_type!(CategoryId, usize, Copy);

utils::strong_type!(PartName, String);
utils::strong_type!(ColorName, String);
utils::strong_type!(CategoryName, String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub category_id: CategoryId,
    pub category_name: CategoryName,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct 
