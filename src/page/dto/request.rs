use serde::Serialize;
use serde_json::{Value, json};

use crate::{
    page::types::PageParent,
    types::{Cover, Icon},
};

#[derive(Serialize)]
pub struct PageCreateBody {
    parent: PageParent,
    // TODO? Vec<Property> -> to_value
    properties: Value,
    icon: Option<Icon>,
    cover: Option<Cover>,
}

impl PageCreateBody {
    pub fn new(parent: PageParent, title: &str) -> Self {
        Self {
            parent,
            properties: json!({"Name": {"title": [{"text": {"content": title}}]}}),
            icon: None,
            cover: None,
        }
    }
}

#[derive(Serialize)]
pub struct PageUpdateBody {
    properties: Value,
}

impl PageUpdateBody {
    pub fn new(properties: Value) -> Self {
        Self { properties }
    }
}
