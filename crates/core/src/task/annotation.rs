use serde::{Deserialize, Serialize};

use super::Date;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    pub entry: Date,
    pub description: String,
}
