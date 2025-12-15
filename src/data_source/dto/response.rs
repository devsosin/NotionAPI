use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    data_source::types::{DataSourceParent, Property},
    database::types::DatabaseParent,
    page::types::Page,
    types::{Cover, Icon, Property as PageProperty, PropertyValue, Title},
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

#[derive(Deserialize, Debug)]
pub struct QueryPageListResponse {
    has_more: bool,
    next_cursor: Option<String>,
    results: Vec<Page>,
}

impl QueryPageListResponse {
    pub fn get_pages(&self) -> Vec<PageInfo> {
        self.results
            .iter()
            .map(|p| PageInfo::new(p.get_id(), p.get_properties()))
            .collect::<Vec<PageInfo>>()
    }
}

pub struct PageInfo {
    id: String,
    properties: Vec<PageProperty>,
}

impl PageInfo {
    fn new(id: &str, properties: Vec<PageProperty>) -> Self {
        Self {
            id: id.into(),
            properties,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_properties(&self) -> &Vec<PageProperty> {
        &self.properties
    }

    // TODO: Vec<> / add find by name
    pub fn find_property(&self, property_value: &PropertyValue) -> Option<String> {
        match self
            .properties
            .iter()
            .find(|p| p.get_property_value().variant_eq(&property_value))
        {
            Some(p) => Some(p.get_property_value().get_value()),
            None => None,
        }
    }
}
