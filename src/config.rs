use config::ConfigError;
use config::FileFormat;
use serde::{Deserialize, Serialize};

use crate::{redis, telemetry};
use crate::server;

pub const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub dry_run: Option<bool>,
    pub redis: redis::Config,
    pub server: server::Config,
    pub telemetry: telemetry::Config,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        pepe_config::load(DEFAULT_CONFIG, FileFormat::Yaml)
    }
}
