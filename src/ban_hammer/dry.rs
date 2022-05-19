use async_trait::async_trait;

use crate::ban_hammer::BanHammer;
use crate::errors::BanError;
use crate::model::BanEntity;

#[derive(Default)]
pub struct DryBanHammer;

#[async_trait]
impl BanHammer for DryBanHammer {
    #[tracing::instrument(skip(self))]
    async fn ban(&self, _be: BanEntity) -> Result<(), BanError> {
        Ok(())
    }
}
