use std::{any::Any, collections::HashMap};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(Box<str>),
    Map(HashMap<Box<str>, Value>),
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
