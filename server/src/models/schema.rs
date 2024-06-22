use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    #[serde(rename = "type")]
    pub property_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    pub title: String,
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub properties: HashMap<String, Properties>,
    pub required: Vec<String>,
}
