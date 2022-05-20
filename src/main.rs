use std::io;

use firewall_executor::ban_hammer::redis::RedisBanHammer;
use firewall_executor::config;
use firewall_executor::redis::get_pool;
use firewall_executor::server::Server;
use firewall_executor::telemetry;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing::info!("start application");

    let cfg = match config::Config::load() {
        Ok(c) => c,
        Err(e) => panic!("can't read config {:?}", e),
    };

    tracing::info!("config loaded; config={:?}", &cfg);
    let subscriber = telemetry::get_subscriber(&cfg.telemetry);
    telemetry::init_subscriber(subscriber);

    let redis_pool = match get_pool(&cfg.redis).await {
        Ok(p) => p,
        Err(e) => panic!("create redis pool {:?}", e),
    };

    let rbh = RedisBanHammer::new(
        redis_pool,
        cfg.redis_query_timeout_secs,
        cfg.redis_keys_prefix.clone(),
        cfg.dry_run.unwrap_or(false),
    );
    let srv = Server::new(&cfg.server, Box::new(rbh))?;
    srv.run().await
}
