use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

use crate::types::{Cover, EditorInfo, Icon, Property};

#[derive(Deserialize, Debug)]
pub struct Page {
    id: String,
    object: String,

    in_trash: bool,
    is_locked: bool,
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

// TODO: macro_rules?
impl Page {
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
pub struct PageParent {
    #[serde(rename(deserialize = "type"))]
    parent_type: String,
    database_id: Option<String>,
    data_source_id: Option<String>,
}
