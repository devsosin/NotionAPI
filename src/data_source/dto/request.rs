use serde::Serialize;
use serde_json::Value;

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

impl From<&(&str, &str)> for Sort {
    fn from(value: &(&str, &str)) -> Self {
        Sort::new(value.0, value.1.into())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl From<&str> for SortDirection {
    fn from(s: &str) -> Self {
        match s {
            "asc" => SortDirection::Ascending,
            "desc" => SortDirection::Descending,
            _ => SortDirection::Descending,
        }
    }
}
