use std::{collections::HashMap, fmt::Debug};

use base64::Engine;
use stride_api::{ConfigSchemaField, ConfigSchemaValue, ConfigValue, config::Error};
use url::Url;
use uuid::Uuid;

// TODO: add a Config builder.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SchemaBuilder {
    id: Box<str>,
    fields: HashMap<Box<str>, ConfigSchemaField>,
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
    pub fn field<T, U, D>(mut self, id: T, name: U, value: D, show_qr_code: bool) -> Self
    where
        T: Into<Box<str>>,
        U: Into<Box<str>>,
        D: Into<ConfigSchemaValue>,
    {
        let id = id.into();
        let name = name.into();

        let inserted = self.fields.insert(
            id.clone(),
            ConfigSchemaField {
                name,
                value: value.into(),
                show_qr_code,
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
    pub fields: HashMap<Box<str>, ConfigSchemaField>,
}

impl Schema {
    pub fn builder<T>(id: T) -> SchemaBuilder
    where
        T: Into<Box<str>>,
    {
        SchemaBuilder::new(id)
    }

    #[must_use]
    pub fn field(&self, id: &str) -> Option<&ConfigSchemaField> {
        self.fields.get(id)
    }

    pub fn with_value_mut<'a, 'b>(
        &'a self,
        config: &'b mut Config,
        id: &str,
    ) -> Option<(&'a ConfigSchemaField, &'b mut ConfigValue)> {
        let value = config.fields.get_mut(id)?;
        let schema = self.field(id)?;
        Some((schema, value))
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub fields: HashMap<Box<str>, ConfigValue>,
}

impl Config {
    pub fn value(&self, id: &str) -> Result<&ConfigValue, Error> {
        let Some(field) = self.fields.get(id) else {
            return Err(Error::MissingField { id: id.into() });
        };
        Ok(field)
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

    pub fn url_value(&self, name: &str) -> Result<&Url, Error> {
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

    pub fn uuid_value(&self, name: &str) -> Result<Uuid, Error> {
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

    pub fn find_field_mut(&mut self, id: &str) -> Option<&mut ConfigValue> {
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
    pub fn get(&self, id: &str) -> Option<&ConfigValue> {
        self.fields.get(id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut ConfigValue> {
        self.fields.get_mut(id)
    }

    pub fn unset(&mut self, id: &str) -> Option<ConfigValue> {
        self.fields.remove(id)
    }

    pub fn set(
        &mut self,
        schema_field: &ConfigSchemaField,
        id: &str,
        value_string: &str,
    ) -> Result<Option<ConfigValue>, Error> {
        let value = match &schema_field.value {
            ConfigSchemaValue::Uuid { .. } => ConfigValue::Uuid {
                value: Uuid::parse_str(value_string).map_err(|error| Error::UuidParse {
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
                value: Url::parse(value_string).map_err(|error| Error::UrlParse {
                    id: id.into(),
                    error,
                })?,
            },
            ConfigSchemaValue::SshKey { .. } => ConfigValue::Uuid {
                value: Uuid::parse_str(value_string).map_err(|error| Error::UuidParse {
                    id: id.into(),
                    error,
                })?,
            },
        };

        Ok(self.fields.insert(id.into(), value))
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackendRecord {
    pub id: Uuid,
    pub name: Box<str>,
    pub enabled: bool,
    pub config: Config,
}
