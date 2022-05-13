use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use std::sync::Arc;
use crate::errors;
use crate::redis::redis_config::RedisConfig;

pub async fn get_pool(cfg: &RedisConfig) -> Result<Pool<RedisConnectionManager>, errors::Redis> {
    let rcm = match RedisConnectionManager::new(cfg.connection_string()) {
        Ok(c) => c,
        Err(re) => return Err(errors::Redis::CreateConnManager(Arc::new(re))),
    };

    match Pool::builder().build(rcm).await {
        Err(re) => Err(errors::Redis::BuildPool(Arc::new(re))),
        Ok(p) => Ok(p),
    }
}
