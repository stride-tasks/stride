use uuid::Uuid;

// TODO: add a Scheme builder.

/// flutter_rust_bridge:ignore
#[derive(thiserror::Error, Debug)]
pub enum SchemaError {
    #[error("missing schema field: {name}")]
    MissingField { name: Box<str> },
    #[error("missing schema field value: {name}")]
    MissingValue { name: Box<str> },
    #[error("type mismatch schema field: expected: {expected}, got: {actual}")]
    TypeMismatch {
        expected: Box<str>,
        actual: Box<str>,
    },
}

/// flutter_rust_bridge:ignore
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "content")]
pub enum Value {
    String(Option<Box<str>>),
    Uuid(Option<Uuid>),
    Bytes(Option<Box<[u8]>>),
}

impl Value {
    #[must_use]
    pub fn as_type_name(&self) -> &str {
        match self {
            Value::String(_) => "string",
            Value::Uuid(_) => "uuid",
            Value::Bytes(_) => "bytes",
        }
    }
}

/// flutter_rust_bridge:ignore
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
// TODO: add an attribute to denote if a property is required.
pub struct Field {
    // TODO: rename to id
    pub id: Box<str>,
    // TODO: rename to name
    pub name: Box<str>,
    pub value: Value,
}

/// flutter_rust_bridge:ignore
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Schema {
    pub title: Box<str>,
    pub fields: Box<[Field]>,
}

impl Schema {
    pub fn field(&self, name: &str) -> Result<&Field, SchemaError> {
        let Some(field) = self.fields.iter().find(|field| field.id.as_ref() == name) else {
            return Err(SchemaError::MissingField { name: name.into() });
        };
        Ok(field)
    }

    pub fn value(&self, name: &str) -> Result<&Value, SchemaError> {
        let Some(field) = self.fields.iter().find(|field| field.id.as_ref() == name) else {
            return Err(SchemaError::MissingField { name: name.into() });
        };
        Ok(&field.value)
    }

    pub fn string_value(&self, name: &str) -> Result<&str, SchemaError> {
        let value = self.value(name)?;
        let Value::String(value) = value else {
            return Err(SchemaError::TypeMismatch {
                expected: "string".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value
            .as_ref()
            .ok_or_else(|| SchemaError::MissingValue { name: name.into() })?;
        Ok(value)
    }

    pub fn uuid_value(&self, name: &str) -> Result<Uuid, SchemaError> {
        let value = self.value(name)?;
        let Value::Uuid(value) = value else {
            return Err(SchemaError::TypeMismatch {
                expected: "uuid".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value.ok_or_else(|| SchemaError::MissingValue { name: name.into() })?;
        Ok(value)
    }

    pub fn bytes_value(&self, name: &str) -> Result<&[u8], SchemaError> {
        let value = self.value(name)?;
        let Value::Bytes(value) = value else {
            return Err(SchemaError::TypeMismatch {
                expected: "bytes".into(),
                actual: value.as_type_name().into(),
            });
        };
        let value = value
            .as_ref()
            .ok_or_else(|| SchemaError::MissingValue { name: name.into() })?;
        Ok(value)
    }
}
