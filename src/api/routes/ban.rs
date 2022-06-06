use std::fmt::{Display, Formatter};

use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::api::http_error::{ErrorResponse, HeaderError};
use crate::api::ANALYZER_HEADER;
use crate::ban_hammer::BanHammerDryRunner;
use crate::model::{BanEntity, BanTarget};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BanRequest {
    pub target: Option<BanTarget>,
    pub reason: Option<String>,
    pub ttl: Option<u32>,
}

impl Display for BanRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Expect because BanRequest derives Serialize
        f.write_str(
            serde_json::to_string(self)
                .expect("BanRequest Display impl")
                .as_str(),
        )
    }
}

#[tracing::instrument(skip(req, hammer))]
#[post("/api/bans")]
pub async fn process_ban(
    req: actix_web::HttpRequest,
    ban_req: web::Json<BanRequest>,
    hammer: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
) -> Result<impl Responder, ErrorResponse> {
    let anl = match req.headers().get(ANALYZER_HEADER) {
        None => {
            return Err(HeaderError::HeaderRequired(ANALYZER_HEADER.to_string()).into());
        }
        Some(s) => match s.to_str() {
            Ok(s) => match s {
                "" => {
                    return Err(HeaderError::HeaderIsEmpty(ANALYZER_HEADER.to_string()).into());
                }
                _ => s,
            },
            Err(e) => {
                tracing::error!("convert analyzer header: {:?}", e);
                return Err(HeaderError::HeaderIsNotString(ANALYZER_HEADER.to_string()).into());
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
            Err(e.into())
        }
    }
}
