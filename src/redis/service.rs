use num_traits::cast::ToPrimitive;
use std::sync::Arc;
use std::time;

use crate::errors;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::cmd;

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

    pub async fn get_ttl(&self, key: String) -> Result<Option<u64>, errors::Redis> {
        tokio::time::timeout(self.timeout, self._get_ttl(key))
            .await
            .map_err(|_| errors::Redis::Timeout)?
    }

    pub async fn _get_ttl<'a>(&self, key: String) -> Result<Option<u64>, errors::Redis> {
        let pool = self.pool.clone();

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return Err(errors::Redis::GetConnection(Arc::new(e)));
            }
        };

        let ttl: i128 = cmd("TTL")
            .arg(&key)
            .query_async(&mut *conn)
            .await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "TTL".to_string()))?;
        match ttl {
            -2 => Err(errors::Redis::KeyNotExist(key)),
            -1 => Err(errors::Redis::NoTTL(key)),
            _ => Ok(Some(ttl.to_u64().unwrap())),
        }
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

        cmd("MULTI")
            .query_async(&mut *conn)
            .await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "MULTI".to_string()))?;

        cmd("HSET")
            .arg(&key)
            .arg(anl)
            .arg(reason)
            .query_async(&mut *conn)
            .await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "HSET".to_string()))?;

        cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .arg("NX")
            .query_async(&mut *conn)
            .await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "EXPIRE NX".to_string()))?;

        cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .arg("GT")
            .query_async(&mut *conn)
            .await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "EXPIRE GT".to_string()))?;

        cmd("EXEC")
            .query_async(&mut *conn)
            .await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "EXEC".to_string()))?;

        Ok(())
    }
}
