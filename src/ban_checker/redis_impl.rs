use crate::ban_checker::BanChecker;
use async_trait::async_trait;

use crate::errors::CheckBanError;
use crate::errors::Redis::KeyNotExist;
use crate::model::BanTarget;
use crate::redis::redis_svc::RedisService;

#[async_trait]
impl BanChecker for RedisService {
    async fn check(&self, bt: &BanTarget) -> Result<Option<u64>, CheckBanError> {
        return match self.get_ttl(bt.value.clone()).await {
            Ok(ttl) => Ok(ttl),
            Err(e) => match e {
                KeyNotExist(_) => Ok(None),
                _ => Err(CheckBanError::Error(e)),
            },
        };
    }
}
