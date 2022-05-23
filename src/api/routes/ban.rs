use crate::api::ANALYZER_HEADER;
use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use tokio::sync::RwLock;

use crate::api::http_error::ErrorResponse;
use crate::ban_hammer::BanHammerDryRunner;
use crate::model::{BanEntity, BanRequest};

#[tracing::instrument(skip(req, hammer))]
#[post("/api/bans")]
pub async fn process_ban(
    req: actix_web::HttpRequest,
    ban_req: web::Json<BanRequest>,
    hammer: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
) -> Result<impl Responder, impl ResponseError> {
    let anl = match req.headers().get(ANALYZER_HEADER) {
        None => {
            return Err(ErrorResponse {
                code: 400,
                reason: format!("{} header required", ANALYZER_HEADER),
                details: None,
            })
        }
        Some(s) => match s.to_str() {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("convert analyzer header: {:?}", e);
                return Err(ErrorResponse {
                    code: 400,
                    reason: format!("can't convert {} header to string", ANALYZER_HEADER),
                    details: None,
                });
            }
        },
    };
    let hammer = hammer.read().await;
    let ban = match BanEntity::new(ban_req.0, anl.to_string()) {
        Ok(b) => b,
        Err(fe) => return Err(fe.into()),
    };
    match hammer.ban(ban).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            Err(ErrorResponse {
                code: 500,
                reason: e.to_string(),
                details: None,
            })
        }
    }
}
