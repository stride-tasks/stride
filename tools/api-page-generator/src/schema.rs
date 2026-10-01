use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub title: String,
    pub description: Option<String>,
    pub properties: std::collections::HashMap<String, Property>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub type_: String,
    pub description: Option<String>,
    pub items: Option<Box<Schema>>,
}

impl Schema {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let schema: Schema = serde_json::from_str(&content)?;
        Ok(schema)
    }
}