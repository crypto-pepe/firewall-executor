use config::ConfigError;
use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

use crate::redis::redis_config::RedisConfig;

use crate::server::server;

pub const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

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
