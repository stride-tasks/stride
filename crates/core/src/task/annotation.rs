use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Date;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Annotation {
    pub id: Uuid,
    pub entry: Date,
    pub text: String,
}

impl Annotation {
    /// Construct annotation with now as the entry date.
    #[must_use]
    pub fn now(text: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            entry: Utc::now(),
            text,
        }
    }
}
