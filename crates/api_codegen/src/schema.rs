use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)] // Evaluates from top to bottom
pub enum SchemaString {
    // 1. Matches if the "enum" key is present in the JSON payload
    Enum {
        #[serde(rename = "enum")]
        enum_values: Vec<String>,
    },

    // 2. Fallback variant if "enum" is missing; looks for type constraints
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)] // Allows parsing variants by fields instead of an explicit "type" tag
pub enum SchemaType {
    Reference {
        #[serde(rename = "$ref")]
        ref_: String,
    },
    Concrete(SchemaConcreteType),
    Any {},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SchemaConcreteType {
    #[serde(rename = "object")]
    Object {
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        properties: HashMap<String, Schema>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        required: Vec<String>,
        #[serde(
            default,
            rename = "additionalProperties",
            skip_serializing_if = "Option::is_none"
        )]
        additional_properties: Option<AdditionalProperties>,
    },
    #[serde(rename = "array")]
    Array { items: Box<Schema> },
    #[serde(rename = "string")]
    String {
        #[serde(flatten)]
        kind: SchemaString,

        #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
        const_value: Option<String>,
    },
    #[serde(rename = "number")]
    Number {
        #[serde(default, rename = "const", skip_serializing_if = "Option::is_none")]
        const_value: Option<f64>,
    },
    #[serde(rename = "integer")]
    Integer {
        #[serde(default, rename = "const", skip_serializing_if = "Option::is_none")]
        const_value: Option<String>,
    },
    #[serde(rename = "boolean")]
    Boolean {
        #[serde(default, rename = "const", skip_serializing_if = "Option::is_none")]
        const_value: Option<String>,
    },
    #[serde(rename = "null")]
    Null {},
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    #[serde(rename = "$id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(flatten)]
    pub schema_type: SchemaType,
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

    #[must_use]
    pub fn explicit_name(&self) -> Option<String> {
        let comment = self.comment.as_deref()?;
        let marker = "@name:";
        let name = comment.strip_prefix(marker)?;
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(trimmed.into())
    }
}
