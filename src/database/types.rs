use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DataSource {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseParent {
    #[serde(rename(deserialize = "type"))]
    parent_type: String,
    page_id: Option<String>,
}
