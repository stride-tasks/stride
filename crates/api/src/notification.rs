use std::{any::Any, collections::HashMap};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(Box<str>),
    Map(HashMap<Box<str>, Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::String(value) => std::fmt::Debug::fmt(value, f),
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
        }
    }
}

pub const PROMPT_METHOD: &str = "stride.notification.prompt";

pub trait Prompt: std::fmt::Debug + Any + 'static {
    fn target(&self) -> Box<str>;
    fn inputs(&self) -> Value {
        Value::Map(HashMap::default())
    }

    fn summary(&self) -> Box<str>;
    fn description(&self) -> Option<Box<str>> {
        None
    }
}

#[derive(Debug)]
pub enum Notification {
    Prompt(Box<dyn Prompt>),
}
