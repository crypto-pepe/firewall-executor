use crate::{errors, BanEntity};
use async_trait::async_trait;

mod redis_cmd;
pub mod redis_impl;
pub(crate) mod redis_pool;

#[async_trait]
pub trait BanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), errors::BanError>;
}
