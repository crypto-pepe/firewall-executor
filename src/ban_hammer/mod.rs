use async_trait::async_trait;
use crate::{BanEntity, errors};

pub mod redis_impl;

#[async_trait]
pub trait BanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), errors::BanError>;
}