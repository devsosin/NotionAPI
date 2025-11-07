use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{errors::ClientError, types::ClientResult};

#[derive(Deserialize, Debug)]
pub struct GetDatabaseResponse {
    id: String,
    object: String,
    data_sources: Vec<DataSource>,
    description: Vec<String>,

    cover: Option<Cover>,
    icon: Option<Icon>,
    public_url: Option<String>,

    created_time: DateTime<Utc>,
    last_edited_time: DateTime<Utc>,

    // common?
    url: String,
    title: Vec<Title>,
    request_id: String,
}

#[derive(Deserialize, Debug)]
pub struct DataSource {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Parent {
    page_id: String,
    #[serde(rename(deserialize = "type"))]
    parent_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Cover {
    #[serde(rename(deserialize = "type"))]
    cover_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Icon {
    #[serde(rename(deserialize = "type"))]
    icon_type: String,
    emoji: Option<String>,
}

// #[serde(flatten)]

#[derive(Deserialize, Debug)]
pub struct NotionListResult {
    object: String,
    has_more: bool,
    next_cursor: Option<String>,
    request_id: String,
    results: Vec<Page>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    id: String,
    object: String,

    in_trash: bool,
    is_locked: bool,
    archived: bool,

    cover: Option<Cover>,
    icon: Option<Icon>,

    created_by: EditorInfo,
    created_time: DateTime<Utc>,
    last_edited_by: EditorInfo,
    last_edited_time: DateTime<Utc>,
    parent: PageParent,

    // 동적 properties 체크..?
    properties: Value,

    public_url: Option<String>,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct PageParent {
    #[serde(rename(deserialize = "type"))]
    parent_type: String,
    database_id: String,
    data_source_id: String,
}

#[derive(Deserialize, Debug)]
pub struct EditorInfo {
    id: String,
    object: String,
}

#[derive(Serialize)]
pub struct QueryBody {
    filter: Value,
    sorts: Vec<Sort>,
}

impl QueryBody {
    pub fn new(filter: Value, sorts: Vec<Sort>) -> Self {
        Self { filter, sorts }
    }
}

#[derive(Serialize)]
pub struct Sort {
    property: String,
    direction: SortDirection,
}

impl Sort {
    pub fn new(property: &str, direction: SortDirection) -> Self {
        Self {
            property: property.into(),
            direction,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Serialize)]
pub struct FilterPart {
    property: Option<String>,
    date: Option<DateFilter>,
    checkbox: Option<CheckboxFilter>,

    or: Option<Vec<FilterPart>>,
    and: Option<Vec<FilterPart>>,
}

impl FilterPart {
    pub fn new_and() -> Self {
        Self {
            property: None,
            date: None,
            checkbox: None,
            or: None,
            and: Some(vec![]),
        }
    }

    pub fn new_filter(property: &str) -> Self {
        Self {
            property: Some(property.into()),
            date: None,
            checkbox: None,
            or: None,
            and: None,
        }
    }

    pub fn get_and(&self) -> &Vec<FilterPart> {
        self.and.as_ref().unwrap().as_ref()
    }

    pub fn add_and_filter(&mut self, filter: FilterPart) -> ClientResult<()> {
        match self.and.as_mut() {
            Some(and) => and.push(filter),
            None => return Err(ClientError::ValidationError("No And Filters".to_string())),
        }

        Ok(())
    }

    pub fn add_or_filter(&mut self, filter: FilterPart) -> ClientResult<()> {
        match self.or.as_mut() {
            Some(or) => or.push(filter),
            None => return Err(ClientError::ValidationError("No And Filters".to_string())),
        }

        Ok(())
    }
}

#[derive(Serialize)]
pub struct DateFilter {
    equlas: Option<DateTime<Utc>>,
    this_week: Option<HashMap<String, String>>,
    before: Option<DateTime<Utc>>,
    after: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct CheckboxFilter {
    equals: bool,
}

// property type title -> Title
#[derive(Deserialize, Debug)]
pub struct Title {
    #[serde(rename(deserialize = "type"))]
    title_type: String,
    text: TitleText,
    annotations: Annotations,
    plain_text: String,
    href: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TitleText {
    content: String,
    link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Annotations {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
    code: bool,
    color: String,
}
