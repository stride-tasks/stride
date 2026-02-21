use serde::{Deserialize, Serialize};

/// A User Defined Attribute (aka UDA).
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uda {
    pub key: String,
    pub value: String,
}
