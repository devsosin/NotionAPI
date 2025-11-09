use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    database::types::{DataSource, DatabaseParent},
    types::{Cover, Icon, Title},
};

#[derive(Deserialize, Debug)]
pub struct GetDatabaseResponse {
    id: String,
    title: Vec<Title>,
    parent: DatabaseParent,
    is_inline: bool,
    in_trash: bool,

    cover: Option<Cover>,
    icon: Option<Icon>,
    public_url: Option<String>,

    data_sources: Vec<DataSource>,
    description: Vec<String>,

    url: String,
    created_time: DateTime<Utc>,
    last_edited_time: DateTime<Utc>,
}
