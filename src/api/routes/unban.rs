use actix_web::web::Data;
use actix_web::{delete, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::api::http_error;
use crate::ban_hammer::BanHammerDryRunner;
use crate::model::UnBanEntity;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnBanRequest {
    pub target: UnBanEntity,
}

#[tracing::instrument(skip(hammer))]
#[delete("/api/bans")]
pub async fn process_unban(
    unban_req: web::Json<UnBanRequest>,
    hammer: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
) -> Result<impl Responder, impl ResponseError> {
    let hammer = hammer.read().await;
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
