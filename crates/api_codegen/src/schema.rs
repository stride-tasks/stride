use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Schema {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    #[serde(rename = "$id")]
    pub id: String,

    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    #[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<SchemaType>,

    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,

    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub const_value: Option<Value>,

    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<AdditionalProperties>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(flatten)]
    pub extras: HashMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaType {
    Single(String),
    Multiple(Vec<String>),
}

impl SchemaType {
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Single(value) => Some(value),
            Self::Multiple(_) => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[allow(variant_size_differences)]
pub enum AdditionalProperties {
    Boolean(bool),
    Schema(Box<Schema>),
}

impl Schema {
    /// Parse a schema from a JSON string.
    ///
    /// # Errors
    /// Returns an error when the JSON payload is invalid or malformed.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}
