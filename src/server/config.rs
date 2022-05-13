use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
pub struct Config {
    pub host: String,
    pub port: u16,
}
