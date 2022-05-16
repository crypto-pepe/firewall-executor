use async_trait::async_trait;

use crate::ban_checker::BanChecker;
use crate::errors::CheckBanError;
use crate::errors::Redis::KeyNotExist;
use crate::redis::Service;

#[async_trait]
impl BanChecker for Service {
    async fn check(&self, bt: String) -> Result<Option<u64>, CheckBanError> {
        return match self.get_ttl(bt).await {
            Ok(ttl) => Ok(ttl),
            Err(e) => match e {
                KeyNotExist(_) => Ok(None),
                _ => Err(CheckBanError::Error(e)),
            },
        };
    }
}
