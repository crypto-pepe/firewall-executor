use config::ConfigError;
use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

use crate::server;

pub(crate) const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
pub struct Config {
    pub redis: RedisConfig,
    pub server: server::Config,
}

impl Config {
    pub fn load(data: &str) -> Result<Self, ConfigError> {
        pepe_config::load(data, config::FileFormat::Yaml)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
#[serde(rename_all = "snake_case")]
pub struct RedisConfig {
    host: String,
    port: u16,
    pub(crate) timeout_sec: u64,
}

impl RedisConfig {
    pub fn connection_string(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}
