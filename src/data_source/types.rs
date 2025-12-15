use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct DataSourceParent {
    #[serde(rename(deserialize = "type"))]
    parent_type: String,
    database_id: String,
}

#[derive(Debug)]
pub struct Property {
    id: String,
    name: String,
    property_info: PropertyInfo,
}

impl From<&Value> for Property {
    fn from(value: &Value) -> Self {
        let property_info: PropertyInfo = value.into();

        Property {
            id: value.get("id").unwrap().to_string(),
            name: value.get("name").unwrap().as_str().unwrap().into(),
            // dsecription: null
            property_info,
        }
    }
}

impl Property {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_property_info(&self) -> &PropertyInfo {
        &self.property_info
    }
}

#[derive(Debug)]
pub enum PropertyInfo {
    ID,
    Verification,
    Title,
    RichText,

    Number,
    Checkbox,
    Date,
    Timestamp,

    Status,
    Select,
    MultiSelect,

    Url,
    Place,
    People,
    PhoneNumber,
    Files,

    Formula,
    Relation,
    Rollup,

    CreatedTime,
    LastEditedTime,
}

impl From<&Value> for PropertyInfo {
    fn from(value: &Value) -> Self {
        let type_str = value.get("type").unwrap().as_str().unwrap();
        // property_value = value.get(type_str)

        // {"type":"select","select":{"options":[{"color":"green","description":null,"id":"816b184a-9dd3-4266-9e2e-16f5cae1c137","name":"작업물"},{"color":"yellow","description":null,"id":"9cf32baa-6966-4200-b955-7d455ac3dea2","name":"레퍼런스"},{"color":"purple","description":null,"id":"bb9c2298-7573-43c1-b78f-d6e6550c2b5c","name":"나중에 보기"}]}}
        // {"type":"select","select":{"options":[{"color":"blue","description":null,"id":"e8f65228-6110-4578-a56c-b2ef8330c27f","name":"다음행동"},{"color":"yellow","description":null,"id":"70e42a58-3f58-4c94-bd2d-03b5ff1f9fdd","name":"일정"},{"color":"default","description":null,"id":"9a05a5c2-e6f7-49c3-b2a8-2e5bed395799","name":"다시 알림"},{"color":"gray","description":null,"id":"13d501cf-60a0-4cf1-b1f4-1c669e4a648c","name":"위임"},{"color":"default","description":null,"id":"304c7ac8-3415-47d9-b514-c471cdb80a6a","name":"언젠가"}]}}
        // {"type":"multi_select","multi_select":{"options":[{"color":"green","description":null,"id":"dbffcf5a-5f12-4b06-8479-a71aa8388c95","name":"컴퓨터"},{"color":"orange","description":null,"id":"9bb90bf9-ceee-4bd0-af89-bcde09aa9100","name":"스마트폰"},{"color":"brown","description":null,"id":"a943cff3-5f1b-4151-8bb2-26eff653c5b6","name":"집에서"},{"color":"blue","description":null,"id":"495bf4a2-e60f-4b6d-815d-cf18900076ad","name":"밖에서"},{"color":"default","description":null,"id":"786f386f-95ab-45eb-9101-a23118e91906","name":"사무실"},{"color":"purple","description":null,"id":"d89894ff-9690-435c-9692-9c5cad62339d","name":"방전"},{"color":"red","description":null,"id":"5869752e-df9f-43e9-aa6a-9da32b383638","name":"창의성"},{"color":"yellow","description":null,"id":"07e636c3-b8e9-4042-9fbc-10cfbebe5df8","name":"단순 노동"}]}}

        // {"type":"formula""formula":{"expression":"{{notion:block_property:Upwn:28714e4a-5157-80cb-8961-000b4d54c831:2d0d6924-cbad-43bd-87a5-cfcf7be2872b}}.join(\"|\")"}}
        // {"type":"formula","formula":{"expression":"{{notion:block_property:%5ETuB:28714e4a-5157-80cb-8961-000b4d54c831:2d0d6924-cbad-43bd-87a5-cfcf7be2872b}}.join(\"|\")"}}
        // {"type":"formula","formula":{"expression":"toNumber(dateBetween(dateEnd({{notion:block_property:d%5Dl%5D:28714e4a-5157-80cb-8961-000b4d54c831:2d0d6924-cbad-43bd-87a5-cfcf7be2872b}}), {{notion:block_property:d%5Dl%5D:28714e4a-5157-80cb-8961-000b4d54c831:2d0d6924-cbad-43bd-87a5-cfcf7be2872b}}, \"minutes\"))"}}
        // {"type":"relation","relation":{"data_source_id":"28714e4a-5157-8087-9cfd-000bf0979848","database_id":"28714e4a-5157-80f2-af8c-e83654e77692","dual_property":{"synced_property_id":"%3CO%7Cd","synced_property_name":"노트"},"type":"dual_property"}}
        // {"type":"relation","relation":{"data_source_id":"28714e4a-5157-80cb-8961-000b4d54c831","database_id":"28714e4a-5157-80d3-8986-d613367fa5a1","single_property":{},"type":"single_property"}}
        // {"type":"relation","relation":{"data_source_id":"28714e4a-5157-80a0-84bf-000b81cda798","database_id":"28714e4a-5157-8054-82e9-df9a469d01b0","dual_property":{"synced_property_id":"eZpb","synced_property_name":"할 일"},"type":"dual_property"}}
        // {"type":"relation","relation":{"data_source_id":"28714e4a-5157-8087-9cfd-000bf0979848","database_id":"28714e4a-5157-80f2-af8c-e83654e77692","dual_property":{"synced_property_id":"ox%7CC","synced_property_name":"할 일"},"type":"dual_property"}}
        // {"type":"rollup","rollup":{"function":"show_original","relation_property_id":"Upwn","relation_property_name":"할일PJT","rollup_property_id":"MD:K","rollup_property_name":"상태"}}

        // {"type":"title","title":{}}
        // {"type":"place","place":{}}
        // {"type":"url","url":{}}
        // {"type":"checkbox","checkbox":{}}
        // {"type":"checkbox","checkbox":{}}
        // {"type":"checkbox","checkbox":{}}
        // {"type":"date","date":{}}
        // {"type":"last_edited_time","last_edited_time":{}}

        match type_str {
            "title" => PropertyInfo::Title,
            "date" => PropertyInfo::Date,
            "checkbox" => PropertyInfo::Checkbox,
            "files" => PropertyInfo::Files,
            "id" => PropertyInfo::ID,
            "multi_select" => PropertyInfo::MultiSelect,
            "number" => PropertyInfo::Number,
            "people" => PropertyInfo::People,
            "phone_number" => PropertyInfo::PhoneNumber,
            "relation" => PropertyInfo::Relation,
            "rich_text" => PropertyInfo::RichText,
            "select" => PropertyInfo::Select,
            "status" => PropertyInfo::Status,
            "timestamp" => PropertyInfo::Timestamp,
            "verification" => PropertyInfo::Verification,
            "place" => PropertyInfo::Place,
            "url" => PropertyInfo::Url,
            "formula" => PropertyInfo::Formula,
            "rollup" => PropertyInfo::Rollup,
            "created_time" => PropertyInfo::CreatedTime,
            "last_edited_time" => PropertyInfo::LastEditedTime,
            _ => panic!("Invalid Notion Property Type"),
        }
    }
}

impl ToString for PropertyInfo {
    fn to_string(&self) -> String {
        match self {
            PropertyInfo::ID => "id",
            PropertyInfo::Title => "title",
            PropertyInfo::Date => "date",
            PropertyInfo::Checkbox => "checkbox",
            PropertyInfo::Files => "files",
            PropertyInfo::MultiSelect => "multi_select",
            PropertyInfo::Number => "number",
            PropertyInfo::People => "people",
            PropertyInfo::PhoneNumber => "phone_number",
            PropertyInfo::Relation => "relation",
            PropertyInfo::RichText => "rich_text",
            PropertyInfo::Select => "select",
            PropertyInfo::Status => "status",
            PropertyInfo::Timestamp => "timestamp",
            PropertyInfo::Verification => "verification",
            PropertyInfo::Place => "place",
            PropertyInfo::Url => "url",
            PropertyInfo::Formula => "formula",
            PropertyInfo::Rollup => "rollup",
            PropertyInfo::CreatedTime => "created_time",
            PropertyInfo::LastEditedTime => "last_edited_time",
        }
        .into()
    }
}
