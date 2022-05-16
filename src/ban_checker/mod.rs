use crate::errors;
use async_trait::async_trait;

pub mod redis_impl;

#[async_trait]
pub trait BanChecker {
    async fn ban_ttl(&self, bt: String) -> Result<Option<u64>, errors::CheckBanError>;
}
