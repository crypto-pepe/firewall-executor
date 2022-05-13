use crate::errors;
use crate::model::BanTarget;
use async_trait::async_trait;

pub mod redis_impl;

#[async_trait]
pub trait BanChecker {
    async fn check(&self, bt: &BanTarget) -> Result<Option<u64>, errors::CheckBanError>;
}
