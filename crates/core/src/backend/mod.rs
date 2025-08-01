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
    pub fn field<T, U, D>(mut self, id: T, name: U, default: D) -> Self
    where
        T: Into<Box<str>>,
        U: Into<Box<str>>,
        D: Into<Value>,
    {
        let id = id.into();
        let name = name.into();

        let inserted = self.fields.insert(
            id.clone(),
            SchemaField {
                name,
                default: default.into(),
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
    pub fn as_config(&self) -> Config {
        let mut fields = HashMap::new();
        for (id, field) in &self.fields {
            fields.insert(id.clone(), field.default.clone());
        }
        Config { fields }
    }

    #[must_use]
    pub fn field(&self, id: &str) -> Option<&SchemaField> {
        self.fields.get(id)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SchemaField {
    pub name: Box<str>,
    pub default: Value,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing schema field: {id}")]
    MissingField { id: Box<str> },
    #[error("missing schema field value: {id}")]
    MissingValue { id: Box<str> },
    #[error("type mismatch schema field: expected: {expected}, got: {actual}")]
    TypeMismatch {
        expected: Box<str>,
        actual: Box<str>,
    },
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EncryptionMode {
    AesOcb256,
}

impl EncryptionMode {
    #[must_use]
    pub fn as_type_name(&self) -> &str {
        match self {
            EncryptionMode::AesOcb256 => "AES(OCB256)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "content")]
pub enum Value {
    String(#[serde(default)] Option<Box<str>>),
    Uuid(#[serde(default)] Option<Uuid>),
    Bytes(#[serde(default)] Option<Box<[u8]>>),
    Url(#[serde(default)] Option<Url>),
    Encryption {
        mode: EncryptionMode,
        #[serde(default)]
        value: Option<Box<[u8]>>,
    },
    SshKey(#[serde(default)] Option<Uuid>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(value) => match value {
                Some(value) => Debug::fmt(value, f),
                None => f.write_str("none"),
            },
            Value::Uuid(uuid) | Value::SshKey(uuid) => match uuid {
                Some(uuid) => Display::fmt(uuid, f),
                None => f.write_str("none"),
            },
            Value::Bytes(value) | Value::Encryption { value, .. } => match value {
                Some(value) => {
                    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(value);
                    Display::fmt(&encoded, f)
                }
                None => f.write_str("none"),
            },
            Value::Url(value) => match value {
                Some(url) => Display::fmt(url, f),
                None => f.write_str("none"),
            },
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
            Value::Encryption { mode, .. } => mode.as_type_name(),
            Value::SshKey(_) => "ssh",
        }
    }

    pub fn string<T>(value: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self::String(Some(value.into()))
    }

    #[must_use]
    pub fn is_some(&self) -> bool {
        match self {
            Value::String(None)
            | Value::Uuid(None)
            | Value::Bytes(None)
            | Value::Url(None)
            | Value::Encryption { value: None, .. }
            | Value::SshKey(None) => false,
            Value::String(_)
            | Value::Uuid(_)
            | Value::Bytes(_)
            | Value::Url(_)
            | Value::Encryption { .. }
            | Value::SshKey(_) => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
                expected: "string".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value
            .as_ref()
            .ok_or_else(|| Error::MissingValue { id: name.into() })?;
        Ok(value)
    }

    pub fn url_value(&self, name: &str) -> Result<&Url, Error> {
        let value = self.value(name)?;
        let Value::Url(value) = value else {
            return Err(Error::TypeMismatch {
                expected: "url".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value
            .as_ref()
            .ok_or_else(|| Error::MissingValue { id: name.into() })?;
        Ok(value)
    }

    pub fn encryption_aes_ocb_256(&self, name: &str) -> Result<&[u8], Error> {
        let value = self.value(name)?;
        let Value::Encryption {
            value,
            mode: EncryptionMode::AesOcb256,
        } = value
        else {
            return Err(Error::TypeMismatch {
                expected: EncryptionMode::AesOcb256.as_type_name().into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value
            .as_ref()
            .ok_or_else(|| Error::MissingValue { id: name.into() })?;
        Ok(value)
    }

    pub fn uuid_value(&self, name: &str) -> Result<Uuid, Error> {
        let value = self.value(name)?;
        let Value::Uuid(value) = value else {
            return Err(Error::TypeMismatch {
                expected: "uuid".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value.ok_or_else(|| Error::MissingValue { id: name.into() })?;
        Ok(value)
    }

    pub fn bytes_value(&self, name: &str) -> Result<&[u8], Error> {
        let value = self.value(name)?;
        let Value::Bytes(value) = value else {
            return Err(Error::TypeMismatch {
                expected: "bytes".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value
            .as_ref()
            .ok_or_else(|| Error::MissingValue { id: name.into() })?;
        Ok(value)
    }

    // TODO: Make this take the known paths.
    pub fn ssh_key_value(&self, name: &str) -> Result<Uuid, Error> {
        let value = self.value(name)?;
        let Value::SshKey(value) = value else {
            return Err(Error::TypeMismatch {
                expected: "ssh_key".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value.ok_or_else(|| Error::MissingValue { id: name.into() })?;
        Ok(value)
    }

    pub fn find_field_mut(&mut self, id: &str) -> Option<&mut Value> {
        self.fields.get_mut(id)
    }

    pub fn align(&self, schema: &Schema) -> Result<Config, Error> {
        let mut config = Config {
            fields: HashMap::default(),
        };

        for (id, field) in &schema.fields {
            let Ok(value) = self.value(id) else {
                config.fields.insert(id.clone(), field.default.clone());
                continue;
            };

            let has_value = !matches!(
                value,
                Value::String(None)
                    | Value::Uuid(None)
                    | Value::Bytes(None)
                    | Value::Url(None)
                    | Value::Encryption { value: None, .. }
                    | Value::SshKey(None)
            );

            // TODO: check if the property matches types.

            if has_value {
                config.fields.insert(id.clone(), value.clone());
            } else {
                config.fields.insert(id.clone(), field.default.clone());
            }
        }
        Ok(config)
    }
}

#[derive(Debug)]
pub struct BackendRecord {
    pub id: Uuid,
    pub name: Box<str>,
    pub enabled: bool,
    pub config: Config,
}
