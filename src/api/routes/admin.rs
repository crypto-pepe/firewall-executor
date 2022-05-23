use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;
use tracing_subscriber::fmt::Formatter;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::EnvFilter;

use crate::ban_hammer::BanHammerDryRunner;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminRequest {
    dry_run: Option<bool>,
    log_level: Option<String>,
}

#[tracing::instrument(skip(bh))]
#[post("/api/config")]
pub async fn admin_settings(
    q: web::Json<AdminRequest>,
    bh: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
    h: Data<Handle<EnvFilter, Formatter>>,
) -> impl Responder {
    let mut bh = bh.write().await;
    if let Some(dry_run) = q.0.dry_run {
        bh.set_dry_run_mode(dry_run);
    }
    if let Some(log_lvl) = q.0.log_level {
        if let Err(e) = h.modify(|e| *e = EnvFilter::new(log_lvl)) {
            return HttpResponse::BadRequest().json(json!({"error":e.to_string()}));
        }
    }

    HttpResponse::Ok().finish()
}
