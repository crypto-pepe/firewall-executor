use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
pub struct Config {
    pub svc_name: String,
    pub jaeger_endpoint: Option<String>,
}
