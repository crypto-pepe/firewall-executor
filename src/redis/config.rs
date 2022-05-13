use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    host: String,
    port: u16,
    pub timeout_sec: u64,
}

impl Config {
    pub fn connection_string(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}
