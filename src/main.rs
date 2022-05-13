extern crate core;

use std::io;

use pepe_log::info;

use crate::ban_hammer::redis_impl::RedisService;
use crate::ban_hammer::redis_pool::get_pool;
use crate::model::{BanEntity, BanRequest};

mod ban_hammer;
mod errors;
#[path = "config.rs"]
mod fw_config;
mod http_error;
mod model;
mod server;

#[tokio::main]
async fn main() -> io::Result<()> {
    info!("start application");

    let cfg = match fw_config::Config::load(fw_config::DEFAULT_CONFIG) {
        Ok(a) => a,
        Err(e) => panic!("can't read config {:?}", e),
    };

    info!("config loaded"; "config" => &cfg);

    let red_pool = match get_pool(&cfg.redis).await {
        Ok(p) => p,
        Err(e) => panic!("create redis pool {:?}", e),
    };

    let red = match RedisService::new(red_pool, cfg.redis.timeout_sec).await {
        Ok(r) => r,
        Err(e) => panic!("can't setup redis {:?}", e),
    };

    let srv = server::Server::new(&cfg.server, red)?;
    srv.run().await
}
