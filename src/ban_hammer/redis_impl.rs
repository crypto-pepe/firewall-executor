use std::sync::Arc;

use async_trait::async_trait;

use crate::ban_hammer::BanHammer;
use crate::errors;
use crate::errors::BanError;
use crate::model::BanEntity;
use crate::redis::Service;

#[async_trait]
impl BanHammer for Service {
    async fn ban(&self, be: BanEntity) -> Result<(), BanError> {
        store(
            self,
            be.target.clone(),
            be.analyzer.clone(),
            be.reason.clone(),
            be.ttl,
        )
        .await
        .map_err(|e| errors::BanError::Error(e))
    }
}

async fn store(
    redis: &Service,
    key: String,
    anl: String,
    reason: String,
    ttl: u32,
) -> Result<(), errors::Redis> {
    tokio::time::timeout(redis.timeout, _store(redis, key, anl, reason, ttl))
        .await
        .map_err(|_| errors::Redis::Timeout)?
}

async fn _store(
    redis: &Service,
    key: String,
    anl: String,
    reason: String,
    ttl: u32,
) -> Result<(), errors::Redis> {
    let pool = redis.pool.clone();

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
