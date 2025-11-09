use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DataSourceParent {
    #[serde(rename(deserialize = "type"))]
    parent_type: String,
    database_id: String,
}
