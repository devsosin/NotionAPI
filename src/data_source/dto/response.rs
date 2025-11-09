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
