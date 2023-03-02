use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use enum_ordinalize::Ordinalize;

#[derive(Ordinalize, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowType {
    Borderless,
    Fullscreen
}

impl Default for WindowType {
    fn default() -> Self {
        Self::Borderless
    }
}

impl From<&JsonValue> for WindowType {
    fn from(value: &JsonValue) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}
