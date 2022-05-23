use crate::dry_runner::DryRunner;
use async_trait::async_trait;

use crate::error;
use crate::model::{BanEntity, UnBanEntity};

pub mod redis;

#[async_trait]
pub trait BanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), error::BanError>;
    async fn unban(&self, t: UnBanEntity) -> Result<(), error::BanError>;
}

pub trait BanHammerDryRunner: BanHammer + DryRunner {}
