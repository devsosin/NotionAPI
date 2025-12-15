use std::mem::discriminant;

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

#[derive(Debug)]
pub struct Property {
    id: String,
    name: String,
    property_value: PropertyValue,
}

impl Property {
    pub fn new(name: &str, value: &Value) -> Self {
        let property_value: PropertyValue = value.into();

        Property {
            id: value.get("id").unwrap().to_string(),
            name: name.into(),
            property_value,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_property_value(&self) -> &PropertyValue {
        &self.property_value
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PropertyValue {
    ID,
    Verification,
    Title(String),
    RichText,

    Number,
    Checkbox(bool),
    Date(String, Option<String>),
    Timestamp,

    Status(String),
    Select(String),
    MultiSelect(Vec<String>),

    Url,
    Place,
    People,
    PhoneNumber,
    Files,

    Formula(String),
    Relation,
    Rollup,

    CreatedTime,
    LastEditedTime,
}

// TODO: BasicProperty Struct (id, name [, color, ...]) MultiSelect
impl PropertyValue {
    pub fn variant_eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }

    pub fn get_value(&self) -> String {
        match self {
            PropertyValue::ID => "id".into(),
            PropertyValue::Title(s) => s.into(),
            PropertyValue::Date(s, e) => {
                let e = match e {
                    Some(end) => "~".to_string() + end,
                    None => "".into(),
                };

                format!("{}{}", s, e)
            }
            PropertyValue::Checkbox(v) => v.to_string(),
            PropertyValue::MultiSelect(values) => values.join("|"),
            PropertyValue::Select(v) => v.clone(),
            PropertyValue::Status(v) => v.clone(),
            PropertyValue::Formula(v) => v.clone(),
            _ => self.to_string(),
        }
    }
}

impl From<&Value> for PropertyValue {
    fn from(value: &Value) -> Self {
        let type_str = value.get("type").unwrap().as_str().unwrap();
        let value = value.get(type_str).unwrap();

        match type_str {
            "title" => {
                let v = value.as_array().unwrap().first().unwrap()["plain_text"]
                    .as_str()
                    .unwrap();
                PropertyValue::Title(v.into())
            }
            "date" => {
                let start = value["start"].as_str().unwrap();
                let end = match value["end"].as_str() {
                    Some(end) => Some(end.into()),
                    None => None,
                };
                PropertyValue::Date(start.into(), end)
            }
            "checkbox" => PropertyValue::Checkbox(value.as_bool().unwrap()),
            "files" => PropertyValue::Files,
            "id" => PropertyValue::ID,
            "multi_select" => {
                let v = value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|item| item["name"].as_str().unwrap().into())
                    .collect::<Vec<String>>();
                PropertyValue::MultiSelect(v)
            }
            "number" => PropertyValue::Number,
            "people" => PropertyValue::People,
            "phone_number" => PropertyValue::PhoneNumber,
            "relation" => PropertyValue::Relation,
            "rich_text" => PropertyValue::RichText,
            "select" => {
                let v = value["name"].as_str().unwrap();
                PropertyValue::Select(v.into())
            }
            "status" => PropertyValue::Status(value["name"].as_str().unwrap().into()),
            "timestamp" => PropertyValue::Timestamp,
            "verification" => PropertyValue::Verification,
            "place" => PropertyValue::Place,
            "url" => PropertyValue::Url,
            "formula" => PropertyValue::Formula(value["string"].as_str().unwrap().into()),
            "rollup" => PropertyValue::Rollup,
            "created_time" => PropertyValue::CreatedTime,
            "last_edited_time" => PropertyValue::LastEditedTime,
            _ => panic!("Invalid Notion Property Type"),
        }
    }
}

impl ToString for &PropertyValue {
    fn to_string(&self) -> String {
        match self {
            PropertyValue::ID => "id",
            PropertyValue::Title(_) => "title",
            PropertyValue::Date(_, _) => "date",
            PropertyValue::Checkbox(_) => "checkbox",
            PropertyValue::Files => "files",
            PropertyValue::MultiSelect(_) => "multi_select",
            PropertyValue::Number => "number",
            PropertyValue::People => "people",
            PropertyValue::PhoneNumber => "phone_number",
            PropertyValue::Relation => "relation",
            PropertyValue::RichText => "rich_text",
            PropertyValue::Select(_) => "select",
            PropertyValue::Status(_) => "status",
            PropertyValue::Timestamp => "timestamp",
            PropertyValue::Verification => "verification",
            PropertyValue::Place => "place",
            PropertyValue::Url => "url",
            PropertyValue::Formula(_) => "formula",
            PropertyValue::Rollup => "rollup",
            PropertyValue::CreatedTime => "created_time",
            PropertyValue::LastEditedTime => "last_edited_time",
        }
        .into()
    }
}
