use indexmap::IndexMap;

pub type Map = IndexMap<Box<str>, Value>;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(Box<str>),
    Bool(bool),
    Array(Vec<Value>),
    Map(Map),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::String(value) => std::fmt::Debug::fmt(value, f),
            Self::Bool(value) => value.fmt(f),
            Self::Array(array) => {
                f.write_str("[")?;
                for (i, value) in array.iter().enumerate() {
                    value.fmt(f)?;
                    if i + 1 != array.len() {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("]")
            }
            Self::Map(map) => {
                f.write_str("{")?;
                for (i, (key, value)) in map.iter().enumerate() {
                    std::fmt::Debug::fmt(key, f)?;
                    f.write_str(": ")?;
                    value.fmt(f)?;
                    if i + 1 != map.len() {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("}")
            }
            Self::Null => f.write_str("null"),
        }
    }
}

#[allow(clippy::missing_errors_doc)]
impl Value {
    /// Create a `Value` from any serializable value.
    ///
    /// # Panics
    /// Panics if the value cannot be serialized to JSON or converted back into `Value`.
    pub fn from_type<T: serde::Serialize>(value: T) -> Self {
        let value = serde_json::to_value(value).expect("Failed to serialize value to JSON");
        serde_json::from_value::<Self>(value).expect("Failed to deserialize value from JSON")
    }

    pub fn to_type<T: for<'de> serde::Deserialize<'de>>(
        &self,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let value = serde_json::to_value(self)?;
        let value = serde_json::from_value::<T>(value)?;
        Ok(value)
    }
}
