use std::fmt::Debug;

use stride_api::Config;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BackendInstance {
    pub id: Uuid,
    pub name: Box<str>,
    pub enabled: bool,
    pub config: Config,
}
