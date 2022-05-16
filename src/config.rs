use std::fs::File;
use std::io::Read;

use config::ConfigError;
use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

use crate::redis;
use crate::server;

pub const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
pub struct Config {
    pub redis: redis::Config,
    pub server: server::Config,
}

impl Config {
    pub fn load(data: &str) -> Result<Self, ConfigError> {
        pepe_config::load(data, config::FileFormat::Yaml)
    }

    pub fn from_file(filename: &str) -> Result<Self, ConfigError> {
        let mut file = File::open(filename).map_err(|e| ConfigError::Message(e.to_string()))?;
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|e| ConfigError::Message(e.to_string()))?;
        pepe_config::load(&*data, config::FileFormat::Yaml)
    }
}
