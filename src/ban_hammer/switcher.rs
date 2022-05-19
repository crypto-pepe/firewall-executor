use async_trait::async_trait;

use crate::ban_hammer::{BanHammer, DryWetBanHammerSwitcher, DryWetSwitcher};
use crate::errors::BanError;
use crate::model::{BanEntity, UnBanEntity};

pub struct DryWetBanHammer {
    is_dry: bool,
    dry: Box<dyn BanHammer + Sync + Send>,
    wet: Box<dyn BanHammer + Sync + Send>,
}

impl DryWetBanHammer {
    pub fn new(
        is_dry: bool,
        dry: Box<dyn BanHammer + Sync + Send>,
        wet: Box<dyn BanHammer + Sync + Send>,
    ) -> Self {
        DryWetBanHammer { is_dry, dry, wet }
    }
}

impl DryWetSwitcher for DryWetBanHammer {
    fn dry(&mut self) {
        self.is_dry = true;
    }
    fn wet(&mut self) {
        self.is_dry = false;
    }
}

#[async_trait]
impl BanHammer for DryWetBanHammer {
    async fn ban(&self, bt: BanEntity) -> Result<(), BanError> {
        if self.is_dry {
            return self.dry.ban(bt).await;
        }
        return self.wet.ban(bt).await;
    }

    async fn unban(&self, t: UnBanEntity) -> Result<(), BanError> {
        if self.is_dry {
            return self.dry.unban(t).await;
        }
        return self.wet.unban(t).await;
    }
}

impl DryWetBanHammerSwitcher for DryWetBanHammer {}
