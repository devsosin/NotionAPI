use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

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
    name: String,
    property_type: PropertyType,
    value: String,
}

impl Property {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_property_type(&self) -> &PropertyType {
        &self.property_type
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_property_from_data_source(value: &Value) -> Self {
        let type_str = value.get("type").unwrap().as_str().unwrap();
        let property_type: PropertyType = type_str.try_into().unwrap();

        Property {
            id: value.get("id").unwrap().to_string(),
            name: value.get("name").unwrap().as_str().unwrap().into(),
            value: "".to_string(),
            property_type,
        }
    }
}

impl From<&Value> for Property {
    fn from(value: &Value) -> Self {
        let type_str = value.get("type").unwrap().as_str().unwrap();
        let property_type: PropertyType = type_str.try_into().unwrap();

        Property {
            id: value.get("id").unwrap().to_string(),
            name: value
                .get("name")
                .unwrap_or(&json!(""))
                .as_str()
                .unwrap()
                .into(),
            value: property_type.get_value(value.get(type_str).unwrap()),
            property_type,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PropertyType {
    Title,
    Date,
    Checkbox,
    Files,
    Formula,
    MultiSelect,
    Number,
    People,
    PhoneNumber,
    Relation,
    RichText,
    Select,
    Status,
    Timestamp,
    Verification,
    Place,
    Url,
    Rollup,

    ID,
    CreatedTime,
    LastEditedTime,
}

impl Into<&str> for &PropertyType {
    fn into(self) -> &'static str {
        match self {
            PropertyType::Title => "title",
            PropertyType::Date => "date",
            PropertyType::Checkbox => "checkbox",
            PropertyType::Files => "files",
            PropertyType::ID => "id",
            PropertyType::MultiSelect => "multi_select",
            PropertyType::Number => "number",
            PropertyType::People => "people",
            PropertyType::PhoneNumber => "phone_number",
            PropertyType::Relation => "relation",
            PropertyType::RichText => "rich_text",
            PropertyType::Select => "select",
            PropertyType::Status => "status",
            PropertyType::Timestamp => "timestamp",
            PropertyType::Verification => "verification",
            PropertyType::Place => "place",
            PropertyType::Url => "url",
            PropertyType::Formula => "formula",
            PropertyType::Rollup => "rollup",
            PropertyType::CreatedTime => "created_time",
            PropertyType::LastEditedTime => "last_edited_time",
        }
    }
}

impl TryFrom<&str> for PropertyType {
    type Error = ClientError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let property_type = match s {
            "title" => PropertyType::Title,
            "date" => PropertyType::Date,
            "checkbox" => PropertyType::Checkbox,
            "files" => PropertyType::Files,
            "id" => PropertyType::ID,
            "multi_select" => PropertyType::MultiSelect,
            "number" => PropertyType::Number,
            "people" => PropertyType::People,
            "phone_number" => PropertyType::PhoneNumber,
            "relation" => PropertyType::Relation,
            "rich_text" => PropertyType::RichText,
            "select" => PropertyType::Select,
            "status" => PropertyType::Status,
            "timestamp" => PropertyType::Timestamp,
            "verification" => PropertyType::Verification,
            "place" => PropertyType::Place,
            "url" => PropertyType::Url,
            "formula" => PropertyType::Formula,
            "rollup" => PropertyType::Rollup,
            "created_time" => PropertyType::CreatedTime,
            "last_edited_time" => PropertyType::LastEditedTime,
            _ => return Err(ClientError::ValidationError("Notion Property Type".into())),
        };

        Ok(property_type)
    }
}

// DataSource에서 가져올 땐 property 설명
// Page에서 가져올 땐 property 값이 들어가있음

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
            // PropertyType::Date => todo!(),
            _ => {
                println!("Value: {:?}", v);
                "".to_string()
            }
        }
    }
}
