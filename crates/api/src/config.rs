use base64::Engine;

use crate::{Config, ConfigField, ConfigSchema, ConfigSchemaField, ConfigSchemaValue, ConfigValue};

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConfigSchemaBuilder {
    id: Box<str>,
    fields: Vec<ConfigSchemaField>,
}

impl ConfigSchemaBuilder {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self {
            id: id.into(),
            fields: Vec::default(),
        }
    }

    /// # Panics
    ///
    /// If the same is field is passed twice.
    #[must_use]
    pub fn field<T, U, D>(mut self, id: T, name: U, value: D, show_qr_code: bool) -> Self
    where
        T: Into<Box<str>>,
        U: Into<Box<str>>,
        D: Into<ConfigSchemaValue>,
    {
        let id = id.into();
        let name = name.into();

        assert!(
            self.fields.iter().filter(|f| f.id == id).count() == 0,
            "field added twice: {id}"
        );

        self.fields.push(ConfigSchemaField {
            id,
            name,
            value: value.into(),
            show_qr_code,
        });
        self
    }

    #[must_use]
    pub fn build(self) -> ConfigSchema {
        ConfigSchema {
            name: self.id,
            fields: self.fields,
        }
    }
}

impl ConfigSchema {
    pub fn builder<T>(id: T) -> ConfigSchemaBuilder
    where
        T: Into<Box<str>>,
    {
        ConfigSchemaBuilder::new(id)
    }

    #[must_use]
    pub fn field(&self, id: &str) -> Option<&ConfigSchemaField> {
        self.fields.iter().find(|f| f.id.as_ref() == id)
    }

    pub fn with_value_mut<'a, 'b>(
        &'a self,
        config: &'b mut Config,
        id: &str,
    ) -> Option<(&'a ConfigSchemaField, &'b mut ConfigValue)> {
        let field = config.fields.iter_mut().find(|f| f.id.as_ref() == id)?;
        let schema = self.field(id)?;
        Some((schema, &mut field.value))
    }
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

impl Default for Config {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}

impl Config {
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&ConfigField> {
        self.fields.iter().find(|field| field.id.as_ref() == id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut ConfigField> {
        self.fields.iter_mut().find(|field| field.id.as_ref() == id)
    }

    pub fn value(&self, id: &str) -> Result<&ConfigValue, Error> {
        let Some(field) = self.get(id) else {
            return Err(Error::MissingField { id: id.into() });
        };
        Ok(&field.value)
    }

    pub fn string_value(&self, name: &str) -> Result<&str, Error> {
        let value = self.value(name)?;
        let ConfigValue::String { value } = value else {
            return Err(Error::TypeMismatch {
                id: name.into(),
                expected: "string".into(),
                actual: value.as_type_name().into(),
            });
        };
        Ok(value)
    }

    pub fn url_value(&self, name: &str) -> Result<&url::Url, Error> {
        let value = self.value(name)?;
        let ConfigValue::Url { value } = value else {
            return Err(Error::TypeMismatch {
                id: name.into(),
                expected: "url".into(),
                actual: value.as_type_name().into(),
            });
        };
        Ok(value)
    }

    pub fn uuid_value(&self, name: &str) -> Result<uuid::Uuid, Error> {
        let value = self.value(name)?;
        let ConfigValue::Uuid { value } = value else {
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
        let ConfigValue::Bytes { value } = value else {
            return Err(Error::TypeMismatch {
                id: name.into(),
                expected: "bytes".into(),
                actual: value.as_type_name().into(),
            });
        };
        Ok(value)
    }

    pub fn align(&self, schema: &ConfigSchema) -> Result<Config, Error> {
        let mut fields = Vec::default();
        for schema_field @ ConfigSchemaField { id, .. } in &schema.fields {
            if let Some(config_field) = self.get(id)
                && let Ok(Some(aligned_value)) = config_field.value.align(schema_field)
            {
                fields.push(ConfigField {
                    id: id.clone(),
                    value: aligned_value,
                });
            }
        }
        Ok(Config { fields })
    }

    pub fn fill(&self, schema: &ConfigSchema) -> Result<Config, Error> {
        let mut fields = self.fields.clone();
        for ConfigSchemaField { id, value, .. } in &schema.fields {
            if self.get(id).is_none()
                && let Some(default) = value.as_value()
            {
                fields.push(ConfigField {
                    id: id.clone(),
                    value: default,
                });
            }
        }
        Ok(Config { fields })
    }

    pub fn unset(&mut self, id: &str) -> Option<ConfigValue> {
        if let Some(pos) = self.fields.iter().position(|field| field.id.as_ref() == id) {
            Some(self.fields.remove(pos).value)
        } else {
            None
        }
    }

    pub fn set(
        &mut self,
        schema_field: &ConfigSchemaField,
        id: &str,
        value_string: &str,
    ) -> Result<Option<ConfigValue>, Error> {
        let value = match &schema_field.value {
            ConfigSchemaValue::Uuid { .. } => ConfigValue::Uuid {
                value: uuid::Uuid::parse_str(value_string).map_err(|error| Error::UuidParse {
                    id: id.into(),
                    error,
                })?,
            },
            ConfigSchemaValue::String { .. } => ConfigValue::String {
                value: value_string.into(),
            },
            ConfigSchemaValue::Bytes { min, max, .. } => {
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
                ConfigValue::Bytes {
                    value: bytes.into_boxed_slice(),
                }
            }
            ConfigSchemaValue::Url { .. } => ConfigValue::Url {
                value: url::Url::parse(value_string).map_err(|error| Error::UrlParse {
                    id: id.into(),
                    error,
                })?,
            },
            ConfigSchemaValue::SshKey { .. } => ConfigValue::Uuid {
                value: uuid::Uuid::parse_str(value_string).map_err(|error| Error::UuidParse {
                    id: id.into(),
                    error,
                })?,
            },
        };

        if let Some(pos) = self.fields.iter().position(|field| field.id.as_ref() == id) {
            let old_value = std::mem::replace(&mut self.fields[pos].value, value);
            Ok(Some(old_value))
        } else {
            self.fields.push(ConfigField {
                id: id.into(),
                value,
            });
            Ok(None)
        }
    }
}
