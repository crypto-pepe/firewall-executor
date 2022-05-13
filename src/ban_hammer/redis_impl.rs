use std::sync::Arc;
use std::time;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use futures_util::future::join_all;
use redis::cmd;
use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;

use crate::ban_hammer::BanHammer;
use crate::errors;
use crate::errors::BanError;
use crate::model::BanEntity;

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
#[serde(rename_all = "snake_case")]
pub struct RedisConfig {
    host: String,
    port: u16,
    timeout_sec: u64,
}

#[derive(Clone)]
pub struct RedisBanHammer {
    pool: Pool<RedisConnectionManager>,
    timeout: time::Duration,
}

impl RedisConfig {
    fn connection_string(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}

#[async_trait]
impl BanHammer for RedisBanHammer {
    async fn ban(&self, be: BanEntity) -> Result<(), BanError> {
        let mut handles = vec![];

        for bt in &be.target {
            handles.push(self.store(
                bt.value.clone(),
                be.analyzer.clone(),
                be.reason.clone(),
                be.ttl,
            ));
        }

        let jh = join_all(handles).await;

        jh.into_iter().collect::<Result<Vec<_>, errors::Redis>>()
            .map_err(|e| errors::BanError::Error(e))?;

        Ok(())
    }
}

impl RedisBanHammer {
    pub async fn new<'a>(cfg: &RedisConfig) -> Result<Self, errors::Redis> {
        let pool = RedisBanHammer::get_pool(cfg).await?;
        let timeout = std::time::Duration::from_secs(cfg.timeout_sec);
        Ok(RedisBanHammer { pool, timeout })
    }

    async fn get_pool(cfg: &RedisConfig) -> Result<Pool<RedisConnectionManager>, errors::Redis> {
        let rcm = match RedisConnectionManager::new(cfg.connection_string()) {
            Ok(c) => c,
            Err(re) => return Err(errors::Redis::CreateConnManager(Arc::new(re))),
        };

        match Pool::builder().build(rcm).await {
            Err(re) => Err(errors::Redis::BuildPool(Arc::new(re))),
            Ok(p) => Ok(p),
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
            .query_async(&mut *conn).await
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
            .query_async(&mut *conn).await
            .map_err(|re| errors::Redis::CMD(Arc::new(re), "EXEC".to_string()))?;

        Ok(())
    }
}
