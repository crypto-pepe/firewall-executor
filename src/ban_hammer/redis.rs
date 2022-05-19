use std::sync::Arc;
use std::time;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

use crate::ban_hammer::BanHammer;
use crate::errors;
use crate::errors::BanError;
use crate::model::{BanEntity, BanTarget, UnBanEntity};

#[derive(Clone)]
pub struct RedisBanHammer {
    pub dry: bool,
    pub pool: Pool<RedisConnectionManager>,
    pub timeout: time::Duration,
}

impl RedisBanHammer {
    pub fn new(pool: Pool<RedisConnectionManager>, timeout_secs: u64, dry: bool) -> Self {
        let timeout = time::Duration::from_secs(timeout_secs);
        RedisBanHammer { pool, timeout, dry }
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

    #[tracing::instrument(skip(self))]
    async fn _del(&self, pattern: String) -> Result<(), errors::Redis> {
        let pool = self.pool.clone();

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return Err(errors::Redis::GetConnection(Arc::new(e)));
            }
        };

        let keys: Vec<String> = conn
            .keys(pattern.clone())
            .await
            .map_err(|e| errors::Redis::GetKeys(Arc::new(e), pattern.clone()))?;
        if !keys.is_empty() {
            conn.del(keys.clone())
                .await
                .map_err(|e| errors::Redis::DeleteKeys(Arc::new(e), keys.clone()))?;
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn del(&self, pattern: String) -> Result<(), errors::Redis> {
        tokio::time::timeout(self.timeout, self._del(pattern))
            .await
            .map_err(|_| errors::Redis::Timeout)?
    }
}

#[async_trait]
impl BanHammer for RedisBanHammer {
    fn dry(&mut self, dry: bool) {
        self.dry = dry
    }

    #[tracing::instrument(skip(self), fields(dry_run = % self.dry))]
    async fn ban(&self, be: BanEntity) -> Result<(), BanError> {
        if self.dry {
            return Ok(());
        }
        self.store(
            be.target.clone(),
            be.analyzer.clone(),
            be.reason.clone(),
            be.ttl,
        )
            .await
            .map_err(errors::BanError::Error)
    }

    #[tracing::instrument(skip(self), fields(dry_run = % self.dry))]
    async fn unban(&self, be: UnBanEntity) -> Result<(), BanError> {
        if self.dry {
            return Ok(());
        }
        match be {
            UnBanEntity::Pattern(p) => {
                if !p.eq("*") {
                    return Err(BanError::NotFound(p));
                }
                self.del(
                    BanTarget {
                        ip: Some("*".to_string()),
                        user_agent: None,
                    }
                        .to_string(),
                )
                    .await
                    .map_err(errors::BanError::Error)?;
                self.del(
                    BanTarget {
                        ip: None,
                        user_agent: Some("*".to_string()),
                    }
                        .to_string(),
                )
                    .await
                    .map_err(errors::BanError::Error)
            }
            UnBanEntity::Target(t) => self
                .del(t.to_string())
                .await
                .map_err(errors::BanError::Error),
        }
    }
}
