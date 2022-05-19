use async_trait::async_trait;

use crate::errors;
use crate::model::{BanEntity, UnBanEntity};

pub mod redis;

#[async_trait]
pub trait BanHammer {
    fn dry(&mut self, dry: bool);
    async fn ban(&self, bt: BanEntity) -> Result<(), errors::BanError>;
    async fn unban(&self, t: UnBanEntity) -> Result<(), errors::BanError>;
}
