use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::Date;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    pub entry: Date,
    pub description: String,
}

impl Annotation {
    /// Construct annotation with now as the entry date.
    #[must_use]
    pub fn now(description: String) -> Self {
        Self {
            entry: Utc::now(),
            description,
        }
    }
}
