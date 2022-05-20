use crate::dry_runner::DryRunner;
use async_trait::async_trait;

use crate::errors;
use crate::model::{BanEntity, UnBanEntity};

pub mod redis;

#[async_trait]
pub trait BanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), errors::BanError>;
    async fn unban(&self, t: UnBanEntity) -> Result<(), errors::BanError>;
}

pub trait BanHammerDryRunner: BanHammer + DryRunner {}
