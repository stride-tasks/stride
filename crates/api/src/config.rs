use base64::Engine;

use crate::{ConfigSchemaField, ConfigSchemaValue, ConfigValue};

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

impl ConfigSchemaValue {
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
    pub fn as_value(&self) -> Option<ConfigValue> {
        match self {
            Self::String { default: None }
            | Self::Uuid { default: None }
            | Self::Bytes { default: None, .. }
            | Self::Url { default: None }
            | Self::SshKey { default: None } => None,
            Self::String {
                default: Some(default),
            } => Some(ConfigValue::String {
                value: default.clone(),
            }),
            Self::Uuid {
                default: Some(default),
            }
            | Self::SshKey {
                default: Some(default),
            } => Some(ConfigValue::Uuid { value: *default }),
            Self::Bytes {
                default: Some(default),
                ..
            } => Some(ConfigValue::Bytes {
                value: default.clone(),
            }),
            Self::Url {
                default: Some(default),
            } => Some(ConfigValue::Url {
                value: default.clone(),
            }),
        }
    }
}

impl std::fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String { value } => std::fmt::Debug::fmt(value, f),
            Self::Uuid { value } => std::fmt::Debug::fmt(value, f),
            Self::Bytes { value } => std::fmt::Debug::fmt(value, f),
            Self::Url { value } => std::fmt::Debug::fmt(value, f),
        }
    }
}

impl ConfigValue {
    #[must_use]
    pub fn as_type_name(&self) -> &str {
        match self {
            Self::String { .. } => "string",
            Self::Uuid { .. } => "uuid",
            Self::Bytes { .. } => "bytes",
            Self::Url { .. } => "url",
        }
    }

    pub fn string<T>(value: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self::String {
            value: value.into(),
        }
    }

    #[must_use]
    pub fn as_value_string(&self) -> Box<str> {
        match self {
            Self::String { value } => value.clone(),
            Self::Uuid { value } => value.to_string().into_boxed_str(),
            Self::Bytes { value } => base64::prelude::BASE64_URL_SAFE
                .encode(value)
                .into_boxed_str(),
            Self::Url { value } => value.to_string().into_boxed_str(),
        }
    }

    #[must_use]
    pub fn as_value_boxed_slice(&self) -> Box<[u8]> {
        match self {
            Self::String { value } => value.as_bytes().to_vec().into_boxed_slice(),
            Self::Uuid { value } => value.as_bytes().to_vec().into_boxed_slice(),
            Self::Bytes { value } => value.clone(),
            Self::Url { value } => value.as_str().as_bytes().to_vec().into_boxed_slice(),
        }
    }

    pub fn align(&self, schema: &ConfigSchemaField) -> Result<Option<Self>, Error> {
        match (&schema.value, self) {
            (ConfigSchemaValue::String { .. }, Self::String { .. })
            | (
                ConfigSchemaValue::Uuid { .. } | ConfigSchemaValue::SshKey { .. },
                Self::Uuid { .. },
            )
            | (ConfigSchemaValue::Url { .. }, Self::Url { .. }) => return Ok(Some(self.clone())),
            (ConfigSchemaValue::Bytes { min, max, .. }, Self::Bytes { value }) => {
                let min = min.unwrap_or(0);
                let max = max.unwrap_or(usize::MAX);
                if !(min..=max).contains(&value.len()) {
                    return Err(Error::OutOfRange {
                        id: schema.name.clone(),
                        min,
                        max,
                        size: value.len(),
                    });
                }
                return Ok(Some(Self::Bytes {
                    value: value.clone(),
                }));
            }
            (ConfigSchemaValue::String { .. }, Self::Uuid { value }) => {
                return Ok(Some(Self::String {
                    value: value.to_string().into_boxed_str(),
                }));
            }
            (ConfigSchemaValue::String { .. }, Self::Url { value }) => {
                return Ok(Some(Self::String {
                    value: value.as_str().into(),
                }));
            }
            (ConfigSchemaValue::Uuid { .. }, Self::String { value }) => {
                match uuid::Uuid::parse_str(&value) {
                    Ok(value) => return Ok(Some(Self::Uuid { value })),
                    Err(error) => {
                        return Err(Error::UuidParse {
                            id: schema.name.clone(),
                            error,
                        });
                    }
                }
            }
            (ConfigSchemaValue::Url { .. }, Self::String { value }) => {
                return match url::Url::parse(&value) {
                    Ok(value) => Ok(Some(Self::Url { value })),
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
