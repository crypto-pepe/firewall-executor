use std::time;

use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use futures_util::future::join_all;
use crate::ban_checker::BanChecker;

use crate::ban_hammer::BanHammer;
use crate::errors;
use crate::errors::{BanError, CheckBanError};
use crate::model::{BanEntity, BanTarget};
use crate::redis::redis_svc::RedisService;

#[async_trait]
impl BanChecker for RedisService {
    async fn check(&self, bt: &BanTarget) -> Result<Option<u64>, CheckBanError> {
        todo!()
    }
}
