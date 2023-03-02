use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::consts::launcher_dir;

pub mod config;

pub mod prelude {
    pub use super::config::prelude::*;

    pub use super::config::Config;
}

use prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FpsUnlocker {
    pub path: PathBuf,
    pub enabled: bool,
    pub config: Config
}

impl Default for FpsUnlocker {
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            path: launcher_dir.join("fps-unlocker"),
            enabled: false,
            config: Config::default()
        }
    }
}

impl From<&JsonValue> for FpsUnlocker {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            path: match value.get("path") {
                Some(value) => match value.as_str() {
                    Some(value) => PathBuf::from(value),
                    None => default.path
                },
                None => default.path
            },

            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            },

            config: match value.get("config") {
                Some(value) => Config::from(value),
                None => default.config
            }
        }
    }
}
