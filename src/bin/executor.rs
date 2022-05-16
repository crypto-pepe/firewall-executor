extern crate core;

use std::{env, io};

use firewall_executor::ban_hammer::redis::RedisBanHammer;
use pepe_log::info;

use firewall_executor::config;
use firewall_executor::redis::get_pool;
use firewall_executor::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    info!("start application");

    let cfg = match env::var("CONFIG_PATH") {
        Ok(cfg_path) => match config::Config::from_file(cfg_path.as_str()) {
            Ok(a) => a,
            Err(e) => panic!("can't read config from file {:?}", e),
        },
        Err(_) => match config::Config::load(config::DEFAULT_CONFIG) {
            Ok(a) => a,
            Err(e) => panic!("can't read default config {:?}", e),
        },
    };

    info!("config loaded"; "config" => &cfg);

    let redis_pool = match get_pool(&cfg.redis).await {
        Ok(p) => p,
        Err(e) => panic!("create redis pool {:?}", e),
    };

    let redis_svc = match RedisBanHammer::new(redis_pool, cfg.redis.timeout_sec).await {
        Ok(r) => r,
        Err(e) => panic!("can't setup redis {:?}", e),
    };

    let srv = Server::new(&cfg.server, redis_svc)?;
    srv.run().await
}
