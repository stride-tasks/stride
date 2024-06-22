use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub uuid: Uuid,
    pub name: String,

    pub search: String,
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterSelection {
    Predefined { uuid: Uuid },
    Temporary { filter: Filter },
}
