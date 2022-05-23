use std::sync::Arc;
use std::time;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

use crate::ban_hammer::{BanHammer, BanHammerDryRunner, DryRunner};
use crate::error;
use crate::error::BanError;
use crate::model::{BanEntity, UnBanEntity};

#[derive(Clone)]
pub struct RedisBanHammer {
    pub dry_run: bool,
    pub pool: Pool<RedisConnectionManager>,
    pub timeout: time::Duration,
    pub namespace: String,
}

impl RedisBanHammer {
    pub fn new(
        pool: Pool<RedisConnectionManager>,
        timeout_secs: u64,
        namespace: String,
        dry_run: bool,
    ) -> Self {
        let timeout = time::Duration::from_secs(timeout_secs);
        RedisBanHammer {
            pool,
            timeout,
            dry_run,
            namespace,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn store(
        &self,
        key: String,
        anl: String,
        reason: String,
        ttl: u32,
    ) -> Result<(), error::Redis> {
        tokio::time::timeout(
            self.timeout,
            self._store(format!("{}{}", self.namespace, key), anl, reason, ttl),
        )
        .await
        .map_err(|_| error::Redis::Timeout)?
    }

    #[tracing::instrument(skip(self))]
    async fn _store(
        &self,
        key: String,
        anl: String,
        reason: String,
        ttl: u32,
    ) -> Result<(), error::Redis> {
        let pool = self.pool.clone();

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return Err(error::Redis::GetConnection(Arc::new(e)));
            }
        };

        #[rustfmt::skip]
        redis::pipe()
            .atomic()
            .hset(&key, anl, reason).ignore()
            .expire(&key, ttl as usize).arg("NX").ignore()
            .expire(&key, ttl as usize).arg("GT").ignore()
            .query_async(&mut *conn).await
            .map_err(|re| error::Redis::Pipeline(
                Arc::from(re),
                vec!["HSET".into(), "EXPIRE NX".into(), "EXPIRE GT".into()]))?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn _del(&self, pattern: String) -> Result<(), error::Redis> {
        let pool = self.pool.clone();

        let mut conn = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return Err(error::Redis::GetConnection(Arc::new(e)));
            }
        };

        let keys: Vec<String> = conn
            .keys(pattern.clone())
            .await
            .map_err(|e| error::Redis::GetKeys(Arc::new(e), pattern.clone()))?;
        if !keys.is_empty() {
            conn.del(keys.clone())
                .await
                .map_err(|e| error::Redis::DeleteKeys(Arc::new(e), keys.clone()))?;
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn del(&self, pattern: String) -> Result<(), error::Redis> {
        tokio::time::timeout(
            self.timeout,
            self._del(format!("{}{}", self.namespace, pattern)),
        )
        .await
        .map_err(|_| error::Redis::Timeout)?
    }
}

impl DryRunner for RedisBanHammer {
    fn set_dry_run_mode(&mut self, mode: bool) {
        self.dry_run = mode
    }
}

#[async_trait]
impl BanHammer for RedisBanHammer {
    #[tracing::instrument(skip(self), fields(dry_run = % self.dry_run))]
    async fn ban(&self, be: BanEntity) -> Result<(), BanError> {
        if self.dry_run {
            tracing::warn!("dry run");
            return Ok(());
        }
        self.store(
            be.target.clone(),
            be.analyzer.clone(),
            be.reason.clone(),
            be.ttl,
        )
        .await
        .map_err(error::BanError::Error)
    }

    #[tracing::instrument(skip(self), fields(dry_run = % self.dry_run))]
    async fn unban(&self, be: UnBanEntity) -> Result<(), BanError> {
        if self.dry_run {
            tracing::warn!("dry run");
            return Ok(());
        }
        match be {
            UnBanEntity::Pattern(p) => {
                if !p.eq("*") {
                    return Err(BanError::NotFound(p));
                }
                self.del(p).await.map_err(error::BanError::Error)
            }
            UnBanEntity::Target(t) => self
                .del(t.to_string())
                .await
                .map_err(error::BanError::Error),
        }
    }
}

impl BanHammerDryRunner for RedisBanHammer {}
