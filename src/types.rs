use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::ClientError;

pub type ClientResult<T> = Result<T, ClientError>;

pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    status: u16,
    code: String,
    object: String,
    message: String,
}

impl ErrorResponse {
    pub fn get_code(&self) -> &str {
        &self.code
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[derive(Deserialize, Debug)]
pub struct NotionResponse<T> {
    object: String,
    request_id: String,
    #[serde(flatten)]
    data: Option<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cover {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    cover_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Icon {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    icon_type: String,
    emoji: Option<String>,
}

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

#[derive(Deserialize, Debug)]
pub struct EditorInfo {
    id: String,
    object: String,
}

pub struct Property {
    id: String,
    name: String,
    property_type: String,
    value: Value,
    // types: people, checkbox, number, rich_text, date, relation, rollup, multi_select, files, select, title
}

impl From<&Value> for Property {
    fn from(value: &Value) -> Self {
        let type_value = value.get("type").unwrap().to_string();

        Property {
            id: value.get("id").unwrap().to_string(),
            name: value.get("name").unwrap().to_string(),
            value: value.get(type_value.clone()).unwrap().to_owned(),
            property_type: type_value,
        }
    }
}
