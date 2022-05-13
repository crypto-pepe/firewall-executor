use async_trait::async_trait;
use crate::errors;
use crate::model::BanEntity;

pub mod redis_impl;

#[async_trait]
pub trait BanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), errors::BanError>;
}
