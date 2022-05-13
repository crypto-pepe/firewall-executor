use std::time;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use futures_util::future::join_all;

use crate::ban_hammer::BanHammer;
use crate::errors;
use crate::errors::BanError;
use crate::model::BanEntity;

#[derive(Clone)]
pub struct RedisService {
    pub(crate) pool: Pool<RedisConnectionManager>,
    pub(crate) timeout: time::Duration,
}

#[async_trait]
impl BanHammer for RedisService {
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

        jh.into_iter()
            .collect::<Result<Vec<_>, errors::Redis>>()
            .map_err(|e| errors::BanError::Error(e))?;

        Ok(())
    }
}
