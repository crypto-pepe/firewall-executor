use async_trait::async_trait;
use crate::errors;
use crate::model::{BanEntity, BanTarget};

pub mod redis_impl;

#[async_trait]
pub trait BanChecker {
    async fn check(&self, bt: &BanTarget) -> Result<Option<u64>, errors::CheckBanError>;
}
