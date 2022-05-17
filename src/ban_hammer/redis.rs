use std::sync::Arc;
use std::time;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

use crate::ban_hammer::BanHammer;
use crate::errors;
use crate::errors::BanError;
use crate::model::BanEntity;

#[async_trait]
impl BanHammer for RedisBanHammer {
    #[tracing::instrument(skip(self))]
    async fn ban(&self, be: BanEntity) -> Result<(), BanError> {
        self.store(
            be.target.clone(),
            be.analyzer.clone(),
            be.reason.clone(),
            be.ttl,
        )
        .await
        .map_err(|e| errors::BanError::Error(e))
    }
}

#[derive(Clone)]
pub struct RedisBanHammer {
    pub pool: Pool<RedisConnectionManager>,
    pub timeout: time::Duration,
}

impl RedisBanHammer {
    pub fn new(pool: Pool<RedisConnectionManager>, timeout_secs: u64) -> Self {
        let timeout = time::Duration::from_secs(timeout_secs);
        RedisBanHammer { pool, timeout }
    }

    #[tracing::instrument(skip(self))]
    async fn store(
        &self,
        key: String,
        anl: String,
        reason: String,
        ttl: u32,
    ) -> Result<(), errors::Redis> {
        tokio::time::timeout(self.timeout, self._store(key, anl, reason, ttl))
            .await
            .map_err(|_| errors::Redis::Timeout)?
    }

    #[tracing::instrument(skip(self))]
    async fn _store(
        &self,
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
