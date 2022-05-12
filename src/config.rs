use config::ConfigError;
use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

use crate::ban_hammer as bh;
use crate::server;

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
pub struct Config {
    pub redis: bh::redis_impl::RedisConfig,
    pub server: server::Config,
}

impl Config {
    pub fn load(data: &str) -> Result<Self, ConfigError> {
        pepe_config::load(data, ::config::FileFormat::Yaml)
    }
}
