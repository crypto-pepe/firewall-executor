use crate::error;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use std::sync::Arc;

pub async fn get_pool(
    cfg: &pepe_config::redis::Config,
) -> Result<Pool<RedisConnectionManager>, error::Redis> {
    let rcm = match RedisConnectionManager::new(cfg.connection_string()) {
        Ok(c) => c,
        Err(re) => return Err(error::Redis::CreateConnManager(Arc::new(re))),
    };

    match Pool::builder().build(rcm).await {
        Err(re) => Err(error::Redis::BuildPool(Arc::new(re))),
        Ok(p) => Ok(p),
    }
}
