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
pub struct NotionResponse<T> {
    object: String,
    request_id: String,
    #[serde(flatten)]
    data: Option<T>,
}

impl<T> NotionResponse<T> {
    pub fn get_data(&self) -> &T {
        self.data.as_ref().unwrap()
    }
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
    // name: String,
    property_type: PropertyType,
    value: String,
    // types: people, checkbox, number, rich_text, date, relation, rollup, multi_select, files, select, title
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PropertyType {
    Title,
    Select,
    MultiSelect,
    Date,
}

impl From<&str> for PropertyType {
    fn from(s: &str) -> Self {
        match s {
            "title" => Self::Title,
            "select" => Self::Select,
            "multi_select" => Self::MultiSelect,
            "date" => Self::Date,
            _ => Self::Title,
        }
    }
}

// multi select 객체는 select를 array로 감쌈 (color, id, name)
impl PropertyType {
    pub fn get_value(&self, v: &Value) -> String {
        match self {
            PropertyType::Title => v.as_array().unwrap().first().unwrap()["plain_text"]
                .as_str()
                .unwrap()
                .into(),
            PropertyType::Select => v["name"].as_str().unwrap().into(),
            PropertyType::MultiSelect => v
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item["name"].as_str().unwrap())
                .collect::<Vec<&str>>()
                .join("|"),
            PropertyType::Date => todo!(),
        }
    }
}

impl From<&Value> for Property {
    fn from(value: &Value) -> Self {
        let type_str = value.get("type").unwrap().as_str().unwrap();
        let property_type: PropertyType = type_str.into();
        // println!("{:?}", value);
        // println!("{:?}", type_value);

        Property {
            id: value.get("id").unwrap().to_string(),
            // name: value.get("name").unwrap(),
            value: property_type.get_value(value.get(type_str).unwrap()),
            property_type: property_type,
        }
    }
}

impl Property {
    pub fn get_value(&self) -> &str {
        &self.value
    }
}
