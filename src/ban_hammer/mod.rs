use crate::errors;
use crate::model::BanEntity;
use async_trait::async_trait;

pub mod redis;

#[async_trait]
pub trait BanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), errors::BanError>;
}
