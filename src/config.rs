use pepe_config::{ConfigError, FileFormat};
use serde::{Deserialize, Serialize};

use crate::server;
use crate::{redis, telemetry};

pub const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub dry_run: Option<bool>,
    pub redis: redis::Config,
    pub server: server::Config,
    pub telemetry: telemetry::Config,
    pub namespace: String,
    pub redis_query_timeout_secs: u64,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        pepe_config::load(DEFAULT_CONFIG, FileFormat::Yaml)
    }
}
