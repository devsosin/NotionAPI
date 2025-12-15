use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{Cover, EditorInfo, Icon, Property};

#[derive(Deserialize, Debug)]
pub struct Page {
    id: String,
    object: String,

    in_trash: Option<bool>,
    is_locked: Option<bool>,
    archived: bool,

    cover: Option<Cover>,
    icon: Option<Icon>,

    parent: PageParent,

    properties: Value,

    public_url: Option<String>,
    url: String,

    created_by: EditorInfo,
    created_time: DateTime<Utc>,
    last_edited_by: EditorInfo,
    last_edited_time: DateTime<Utc>,
}

impl Page {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_properties(&self) -> Vec<Property> {
        self.properties
            .as_object()
            .unwrap()
            .iter()
            .map(|(_, v)| v.into())
            .collect::<Vec<Property>>()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ParentType {
    DatabaseId,
    DataSourceId,
    PageId,
}

impl From<&str> for ParentType {
    fn from(s: &str) -> Self {
        match s {
            "page" => ParentType::PageId,
            "database" => ParentType::DatabaseId,
            _ => ParentType::DataSourceId,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageParent {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    parent_type: ParentType,
    database_id: Option<String>,
    data_source_id: Option<String>,
    page_id: Option<String>,
}

impl PageParent {
    pub fn new(type_str: &str, id: &str) -> Self {
        match Into::<ParentType>::into(type_str) {
            ParentType::DataSourceId => Self::new_datasource(id),
            ParentType::PageId => Self::new_page(id),
            ParentType::DatabaseId => Self::new_database(id),
        }
    }

    pub fn new_datasource(id: &str) -> Self {
        Self {
            parent_type: ParentType::DataSourceId,
            database_id: None,
            data_source_id: Some(id.into()),
            page_id: None,
        }
    }
    pub fn new_database(id: &str) -> Self {
        Self {
            parent_type: ParentType::DatabaseId,
            database_id: Some(id.into()),
            data_source_id: None,
            page_id: None,
        }
    }
    pub fn new_page(id: &str) -> Self {
        Self {
            parent_type: ParentType::PageId,
            database_id: None,
            data_source_id: None,
            page_id: Some(id.into()),
        }
    }
}
