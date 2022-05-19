use async_trait::async_trait;

use crate::errors;
use crate::model::BanEntity;

pub mod dry;
pub mod redis;
pub mod switcher;

#[async_trait]
pub trait BanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), errors::BanError>;
}

pub trait DryWetSwitcher {
    fn dry(&mut self);
    fn wet(&mut self);
}

pub trait DryWetBanHammerSwitcher: BanHammer + DryWetSwitcher {}
