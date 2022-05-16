extern crate core;

use std::io;

use pepe_log::info;

use firewall_executor::config;
use firewall_executor::redis::get_pool;
use firewall_executor::redis::Service;
use firewall_executor::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    info!("start application");

    let cfg = match config::Config::load(config::DEFAULT_CONFIG) {
        Ok(a) => a,
        Err(e) => panic!("can't read config {:?}", e),
    };

    info!("config loaded"; "config" => &cfg);

    let redis_pool = match get_pool(&cfg.redis).await {
        Ok(p) => p,
        Err(e) => panic!("create redis pool {:?}", e),
    };

    let red = match Service::new(redis_pool, cfg.redis.timeout_sec).await {
        Ok(r) => r,
        Err(e) => panic!("can't setup redis {:?}", e),
    };

    let srv = Server::new(&cfg.server, red, Some(true), Some(false))?;
    srv.run().await
}
