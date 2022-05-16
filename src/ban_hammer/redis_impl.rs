use async_trait::async_trait;

use crate::ban_hammer::BanHammer;
use crate::errors;
use crate::errors::BanError;
use crate::model::BanEntity;
use crate::redis::Service;

#[async_trait]
impl BanHammer for Service {
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
