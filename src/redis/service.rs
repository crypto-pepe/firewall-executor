use std::sync::Arc;
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

    pub async fn store(
        self: &Self,
        key: String,
        anl: String,
        reason: String,
        ttl: u32,
    ) -> Result<(), errors::Redis> {
        tokio::time::timeout(self.timeout, self._store(key, anl, reason, ttl))
            .await
            .map_err(|_| errors::Redis::Timeout)?
    }

    async fn _store(
        self: &Self,
        key: String,
        anl: String,
        reason: String,
        ttl: u32,
    ) -> Result<(), errors::Redis> {
        let pool = self.pool.clone();

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return Err(errors::Redis::GetConnection(Arc::new(e)));
            }
        };

        #[rustfmt::skip]
        redis::pipe()
            .atomic()
            .hset(&key, anl, reason).ignore()
            .expire(&key, ttl as usize).arg("NX").ignore()
            .expire(&key, ttl as usize).arg("GT").ignore()
            .query_async(&mut *conn).await
            .map_err(|re| errors::Redis::Pipeline(
                Arc::from(re),
                vec!["HSET".into(), "EXPIRE NX".into(), "EXPIRE GT".into()]))?;

        Ok(())
    }
}
