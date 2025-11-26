use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    data_source::types::DataSourceParent,
    database::types::DatabaseParent,
    page::types::Page,
    types::{Cover, Icon, Property, Title},
};

#[derive(Deserialize, Debug)]
pub struct GetDataSourceResponse {
    id: String,
    title: Vec<Title>,
    archived: bool,
    is_inline: bool,

    cover: Option<Cover>,
    icon: Option<Icon>,

    parent: DataSourceParent,
    database_parent: DatabaseParent,

    properties: Value,

    url: String,
    created_time: DateTime<Utc>,
    last_edited_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct QueryPageListResponse {
    has_more: bool,
    next_cursor: Option<String>,
    results: Vec<Page>,
}

pub struct PageInfo {
    id: String,
    properties: Vec<Property>,
}

impl PageInfo {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_properties(&self) -> &Vec<Property> {
        &self.properties
    }
}

impl QueryPageListResponse {
    // TODO: format Page {id, properties}
    pub fn get_pages(&self) -> Vec<PageInfo> {
        self.results
            .iter()
            .map(|p| PageInfo {
                id: p.get_id().into(),
                properties: p.get_properties(),
            })
            .collect::<Vec<PageInfo>>()
    }
}

// TODO: macro_rules?
impl GetDataSourceResponse {
    pub fn get_properties(&self) -> Vec<Property> {
        self.properties
            .as_object()
            .unwrap()
            .iter()
            .map(|(_, v)| v.into())
            .collect::<Vec<Property>>()
    }
}

pub trait PropertyParser {
    fn parse_property(&self) -> Vec<Property>;
}
