use pepe_config::{ConfigError, FileFormat};
use serde::{Deserialize, Serialize};

use crate::server;
use crate::telemetry;

pub const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub dry_run: Option<bool>,
    pub redis: pepe_config::redis::Config,
    pub server: server::Config,
    pub telemetry: telemetry::Config,
    pub redis_keys_prefix: String,
    #[serde(default = "default_redis_query_timeout_secs")]
    pub redis_query_timeout_secs: u64,
}

fn default_redis_query_timeout_secs() -> u64 {
    5
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        pepe_config::load(DEFAULT_CONFIG, FileFormat::Yaml)
    }
}
