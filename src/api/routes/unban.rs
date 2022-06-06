use actix_web::web::Data;
use actix_web::{delete, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::api::http_error;
use crate::api::http_error::UnBanRequestConversionError;
use crate::api::routes::unban::UnBanRequestConversionError::{
    IPOrUserAgentRequired, PatternUnsupported,
};
use crate::ban_hammer::BanHammerDryRunner;
use crate::model::UnBanEntity;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnBanRequest {
    pub target: UnBanEntity,
}

impl UnBanRequest {
    pub fn verify(&self) -> Result<(), UnBanRequestConversionError> {
        match &self.target {
            UnBanEntity::Target(t) => {
                if t.ip.is_none() && t.user_agent.is_none() {
                    Err(IPOrUserAgentRequired)
                } else {
                    Ok(())
                }
            }
            UnBanEntity::Pattern(p) => {
                if !p.eq("*") {
                    Err(PatternUnsupported)
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[tracing::instrument(skip(hammer))]
#[delete("/api/bans")]
pub async fn process_unban(
    unban_req: web::Json<UnBanRequest>,
    hammer: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
) -> Result<impl Responder, impl ResponseError> {
    let hammer = hammer.read().await;
    if let Err(e) = unban_req.verify() {
        return Err(e.into());
    }
    match hammer.unban(unban_req.0.target).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            Err(http_error::ErrorResponse {
                code: 500,
                reason: e.to_string(),
                details: None,
            })
        }
    }
}
