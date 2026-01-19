use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use base64::Engine;
use url::Url;
use uuid::Uuid;

// TODO: add a Config builder.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SchemaBuilder {
    id: Box<str>,
    fields: HashMap<Box<str>, SchemaField>,
}

impl SchemaBuilder {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self {
            id: id.into(),
            fields: HashMap::default(),
        }
    }

    /// # Panics
    ///
    /// If the same is field is passed twice.
    #[must_use]
    pub fn field<T, U, D>(mut self, id: T, name: U, value: D) -> Self
    where
        T: Into<Box<str>>,
        U: Into<Box<str>>,
        D: Into<SchemaValue>,
    {
        let id = id.into();
        let name = name.into();

        let inserted = self.fields.insert(
            id.clone(),
            SchemaField {
                name,
                value: value.into(),
            },
        );
        assert!(inserted.is_none(), "field added twice: {id}");
        self
    }

    #[must_use]
    pub fn build(self) -> Schema {
        Schema {
            name: self.id,
            fields: self.fields,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Schema {
    pub name: Box<str>,
    pub fields: HashMap<Box<str>, SchemaField>,
}

impl Schema {
    pub fn builder<T>(id: T) -> SchemaBuilder
    where
        T: Into<Box<str>>,
    {
        SchemaBuilder::new(id)
    }

    #[must_use]
    pub fn field(&self, id: &str) -> Option<&SchemaField> {
        self.fields.get(id)
    }

    pub fn with_value_mut<'a, 'b>(
        &'a self,
        config: &'b mut Config,
        id: &str,
    ) -> Option<(&'a SchemaField, &'b mut Value)> {
        let value = config.fields.get_mut(id)?;
        let schema = self.field(id)?;
        Some((schema, value))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum SchemaValue {
    String {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<Box<str>>,
    },
    Uuid {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<Uuid>,
    },
    Bytes {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "::serde_with::As::<Option<serde_with::base64::Base64>>")]
        default: Option<Box<[u8]>>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<usize>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<usize>,
    },
    Url {
        #[serde(default)]
        default: Option<Url>,
    },
    // TODO: Should SshKey variant be a Uuid type with a SshKey category type?
    SshKey {
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<Uuid>,
    },
}

impl SchemaValue {
    #[must_use]
    pub fn as_type_name(&self) -> &str {
        match self {
            Self::String { .. } => "string",
            Self::Uuid { .. } => "uuid",
            Self::Bytes { .. } => "bytes",
            Self::Url { .. } => "url",
            Self::SshKey { .. } => "ssh-key",
        }
    }

    #[must_use]
    pub fn as_value(&self) -> Option<Value> {
        match self {
            SchemaValue::String { default: None }
            | SchemaValue::Uuid { default: None }
            | SchemaValue::Bytes { default: None, .. }
            | SchemaValue::Url { default: None }
            | SchemaValue::SshKey { default: None } => None,
            SchemaValue::String {
                default: Some(default),
            } => Some(Value::String(default.clone())),
            SchemaValue::Uuid {
                default: Some(default),
            }
            | SchemaValue::SshKey {
                default: Some(default),
            } => Some(Value::Uuid(*default)),
            SchemaValue::Bytes {
                default: Some(default),
                ..
            } => Some(Value::Bytes(default.clone())),
            SchemaValue::Url {
                default: Some(default),
            } => Some(Value::Url(default.clone())),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SchemaField {
    pub name: Box<str>,
    #[serde(flatten)]
    pub value: SchemaValue,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing schema field: {id}")]
    MissingField { id: Box<str> },
    #[error("missing config field: {id}")]
    MissingValue { id: Box<str> },
    #[error("property {id} does not match type, expected {expected} got {actual}")]
    TypeMismatch {
        id: Box<str>,
        expected: Box<str>,
        actual: Box<str>,
    },
    #[error("property {id} is out of range: expected [{min}, {max}] (inclusive), got {size}")]
    OutOfRange {
        id: Box<str>,
        min: usize,
        max: usize,
        size: usize,
    },
    #[error("property {id} invalid uuid: {error}")]
    UuidParse {
        id: Box<str>,
        #[source]
        error: uuid::Error,
    },
    #[error("property {id} invalid url: {error}")]
    UrlParse {
        id: Box<str>,
        #[source]
        error: url::ParseError,
    },
    #[error("property {id} invalid base64: {error}")]
    Base64Decode {
        id: Box<str>,
        #[source]
        error: base64::DecodeError,
    },
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Value {
    String(Box<str>),
    Uuid(Uuid),
    Bytes(#[serde(with = "::serde_with::As::<serde_with::base64::Base64>")] Box<[u8]>),
    Url(Url),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(value) => Debug::fmt(value, f),
            Value::Uuid(value) => Debug::fmt(value, f),
            Value::Bytes(value) => Debug::fmt(value, f),
            Value::Url(value) => Debug::fmt(value, f),
        }
    }
}

impl Value {
    #[must_use]
    pub fn as_type_name(&self) -> &str {
        match self {
            Value::String(_) => "string",
            Value::Uuid(_) => "uuid",
            Value::Bytes(_) => "bytes",
            Value::Url(_) => "url",
        }
    }

    pub fn string<T>(value: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self::String(value.into())
    }

    #[must_use]
    pub fn as_value_string(&self) -> Box<str> {
        match self {
            Value::String(v) => v.clone(),
            Value::Uuid(v) => v.to_string().into_boxed_str(),
            Value::Bytes(v) => base64::prelude::BASE64_URL_SAFE.encode(v).into_boxed_str(),
            Value::Url(v) => v.to_string().into_boxed_str(),
        }
    }

    #[must_use]
    pub fn as_value_boxed_slice(&self) -> Box<[u8]> {
        match self {
            Value::String(v) => v.as_bytes().to_vec().into_boxed_slice(),
            Value::Uuid(v) => v.as_bytes().to_vec().into_boxed_slice(),
            Value::Bytes(v) => v.clone(),
            Value::Url(v) => v.as_str().as_bytes().to_vec().into_boxed_slice(),
        }
    }

    pub fn align(&self, schema: &SchemaField) -> Result<Option<Self>, Error> {
        match (&schema.value, self) {
            (SchemaValue::String { .. }, Value::String(_))
            | (SchemaValue::Uuid { .. } | SchemaValue::SshKey { .. }, Value::Uuid(_))
            | (SchemaValue::Url { .. }, Value::Url(_)) => return Ok(Some(self.clone())),
            (SchemaValue::Bytes { min, max, .. }, Value::Bytes(bytes)) => {
                let min = min.unwrap_or(0);
                let max = max.unwrap_or(usize::MAX);
                if !(min..=max).contains(&bytes.len()) {
                    return Err(Error::OutOfRange {
                        id: schema.name.clone(),
                        min,
                        max,
                        size: bytes.len(),
                    });
                }
                return Ok(Some(Value::Bytes(bytes.clone())));
            }
            (SchemaValue::String { .. }, Value::Uuid(uuid)) => {
                return Ok(Some(Value::String(uuid.to_string().into_boxed_str())));
            }
            (SchemaValue::String { .. }, Value::Url(url)) => {
                return Ok(Some(Value::String(url.as_str().into())));
            }
            (SchemaValue::Uuid { .. }, Value::String(value)) => match Uuid::parse_str(value) {
                Ok(value) => return Ok(Some(Value::Uuid(value))),
                Err(error) => {
                    return Err(Error::UuidParse {
                        id: schema.name.clone(),
                        error,
                    });
                }
            },
            (SchemaValue::Url { .. }, Value::String(value)) => {
                return match Url::parse(value) {
                    Ok(value) => Ok(Some(Value::Url(value))),
                    Err(error) => Err(Error::UrlParse {
                        id: schema.name.clone(),
                        error,
                    }),
                };
            }
            (_, _) => {}
        }

        Err(Error::TypeMismatch {
            id: schema.name.clone(),
            expected: schema.value.as_type_name().into(),
            actual: self.as_type_name().into(),
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub fields: HashMap<Box<str>, Value>,
}

impl Config {
    pub fn value(&self, id: &str) -> Result<&Value, Error> {
        let Some(field) = self.fields.get(id) else {
            return Err(Error::MissingField { id: id.into() });
        };
        Ok(field)
    }

    pub fn string_value(&self, name: &str) -> Result<&str, Error> {
        let value = self.value(name)?;
        let Value::String(value) = value else {
            return Err(Error::TypeMismatch {
                id: name.into(),
                expected: "string".into(),
                actual: value.as_type_name().into(),
            });
        };
        Ok(value)
    }

    pub fn url_value(&self, name: &str) -> Result<&Url, Error> {
        let value = self.value(name)?;
        let Value::Url(value) = value else {
            return Err(Error::TypeMismatch {
                id: name.into(),
                expected: "url".into(),
                actual: value.as_type_name().into(),
            });
        };
        Ok(value)
    }

    pub fn uuid_value(&self, name: &str) -> Result<Uuid, Error> {
        let value = self.value(name)?;
        let Value::Uuid(value) = value else {
            return Err(Error::TypeMismatch {
                id: name.into(),
                expected: "uuid".into(),
                actual: value.as_type_name().into(),
            });
        };
        Ok(*value)
    }

    pub fn bytes_value(&self, name: &str) -> Result<&[u8], Error> {
        let value = self.value(name)?;
        let Value::Bytes(value) = value else {
            return Err(Error::TypeMismatch {
                id: name.into(),
                expected: "bytes".into(),
                actual: value.as_type_name().into(),
            });
        };
        Ok(value)
    }

    pub fn find_field_mut(&mut self, id: &str) -> Option<&mut Value> {
        self.fields.get_mut(id)
    }

    pub fn align(&self, schema: &Schema) -> Result<Config, Error> {
        let mut fields = HashMap::default();
        for (name, field) in &schema.fields {
            if let Some(value) = self.get(name)
                && let Ok(Some(aligned_value)) = value.align(field)
            {
                fields.insert(name.clone(), aligned_value);
            }
        }
        Ok(Config { fields })
    }

    pub fn fill(&self, schema: &Schema) -> Result<Config, Error> {
        let mut fields = self.fields.clone();
        for (name, field) in &schema.fields {
            if self.get(name).is_none()
                && let Some(default) = field.value.as_value()
            {
                fields.insert(name.clone(), default);
            }
        }
        Ok(Config { fields })
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Value> {
        self.fields.get(id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Value> {
        self.fields.get_mut(id)
    }

    pub fn unset(&mut self, id: &str) -> Option<Value> {
        self.fields.remove(id)
    }

    pub fn set(
        &mut self,
        schema_field: &SchemaField,
        id: &str,
        value_string: &str,
    ) -> Result<Option<Value>, Error> {
        let value = match &schema_field.value {
            SchemaValue::Uuid { .. } => Value::Uuid(Uuid::parse_str(value_string).map_err(
                |error| Error::UuidParse {
                    id: id.into(),
                    error,
                },
            )?),
            SchemaValue::String { .. } => Value::String(value_string.into()),
            SchemaValue::Bytes { min, max, .. } => {
                let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(value_string)
                    .map_err(|error| Error::Base64Decode {
                        id: id.into(),
                        error,
                    })?;
                let min = min.unwrap_or(0);
                let max = max.unwrap_or(usize::MAX);

                if !(min..=max).contains(&bytes.len()) {
                    return Err(Error::OutOfRange {
                        id: id.into(),
                        min,
                        max,
                        size: bytes.len(),
                    });
                }
                Value::Bytes(bytes.into_boxed_slice())
            }
            SchemaValue::Url { .. } => {
                Value::Url(Url::parse(value_string).map_err(|error| Error::UrlParse {
                    id: id.into(),
                    error,
                })?)
            }
            SchemaValue::SshKey { .. } => Value::Uuid(Uuid::parse_str(value_string).map_err(
                |error| Error::UuidParse {
                    id: id.into(),
                    error,
                },
            )?),
        };

        Ok(self.fields.insert(id.into(), value))
    }
}

#[derive(Debug)]
pub struct BackendRecord {
    pub id: Uuid,
    pub name: Box<str>,
    pub enabled: bool,
    pub config: Config,
}
