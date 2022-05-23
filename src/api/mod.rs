pub use config::Config;
pub use server::Server;

pub mod config;
pub mod http_error;
pub mod routes;
#[allow(clippy::module_inception)]
pub mod server;

const ANALYZER_HEADER: &str = "X-Analyzer-Id";
