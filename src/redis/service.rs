use std::time;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;

use crate::errors;

#[derive(Clone)]
pub struct Service {
    pub pool: Pool<RedisConnectionManager>,
    pub timeout: time::Duration,
}

impl Service {
    pub async fn new(
        pool: Pool<RedisConnectionManager>,
        timeout_secs: u64,
    ) -> Result<Self, errors::Redis> {
        let timeout = time::Duration::from_secs(timeout_secs);
        Ok(Service { pool, timeout })
    }
}
