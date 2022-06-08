mod api;
mod ban_hammer;
mod config;
mod dry_runner;
mod error;
mod model;
mod redis;
mod telemetry;

use api::Server;
use ban_hammer::redis::RedisBanHammer;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing::info!("start application");

    let cfg = match config::Config::load() {
        Ok(c) => c,
        Err(e) => panic!("can't read config {:?}", e),
    };

    tracing::info!("config loaded; config={:?}", &cfg);
    let (subscriber, log_filter_handler) = telemetry::get_subscriber(&cfg.telemetry);
    telemetry::init_subscriber(subscriber);

    let redis_pool = match redis::get_pool(&cfg.redis).await {
        Ok(p) => p,
        Err(e) => panic!("create redis pool {:?}", e),
    };

    let rbh = RedisBanHammer::new(
        redis_pool,
        cfg.redis_query_timeout_secs,
        cfg.redis_keys_prefix.clone(),
        cfg.dry_run.unwrap_or(false),
    );
    let srv = Server::new(&cfg.server, Box::new(rbh), log_filter_handler)?;
    srv.run().await
}
