extern crate core;

use std::io;

use pepe_log::info;

use crate::ban_hammer::redis_impl::RedisBanHammer;
use crate::model::{BanEntity, BanRequest};

#[path = "config.rs"]
mod fw_config;
mod http_error;
mod model;
mod server;
mod errors;
mod ban_hammer;

const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

#[tokio::main]
async fn main() -> io::Result<()> {
    info!("start application");

    let cfg = match fw_config::Config::load(DEFAULT_CONFIG)
    {
        Ok(a) => a,
        Err(e) => panic!("can't read config {:?}", e),
    };

    info!("config loaded"; "config" => &cfg);

    let red = match RedisBanHammer::new(&cfg.redis).await {
        Ok(r) => r,
        Err(e) => panic!("can't setup redis {:?}", e),
    };

    let srv = server::Server::new(&cfg.server, red)?;
    srv.run().await
}