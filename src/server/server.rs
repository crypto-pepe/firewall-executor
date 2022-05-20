use actix_web::web::Data;
use actix_web::{
    delete, dev, error, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder,
};
use mime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, RwLock};
use tokio::io;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::Formatter;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::EnvFilter;

use crate::ban_hammer::BanHammerDryRunner;
use crate::model::{BanEntity, BanRequest, UnBanRequest};
use crate::server::Config;
use crate::ANALYZER_HEADER;

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(
        cfg: &Config,
        bh: Box<dyn BanHammerDryRunner + Sync + Send>,
        log_filter_handler: Handle<EnvFilter, Formatter>,
    ) -> Result<Server, io::Error> {
        let bh = Data::from(Arc::new(RwLock::new(bh)));
        let lfh = Data::from(Arc::new(log_filter_handler));

        let srv = HttpServer::new(move || {
            App::new()
                .app_data(bh.clone())
                .app_data(lfh.clone())
                .configure(server_config())
                .wrap(TracingLogger::default())
        });

        let srv = srv.bind((cfg.host.clone(), cfg.port))?.run();
        Ok(Server { srv })
    }

    pub async fn run(self) -> io::Result<()> {
        self.srv.await
    }
}

fn server_config() -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg| {
        let json_cfg = web::JsonConfig::default()
            .content_type(|mime| mime == mime::APPLICATION_JSON)
            .error_handler(|err, _| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().into()).into()
            });
        cfg.app_data(json_cfg)
            .service(process_ban)
            .service(admin_settings)
            .service(process_unban);
    })
}

#[tracing::instrument(skip(req, hammer))]
#[post("/api/bans")]
async fn process_ban(
    req: actix_web::HttpRequest,
    ban_req: web::Json<BanRequest>,
    hammer: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
) -> impl Responder {
    let anl = match req.headers().get(ANALYZER_HEADER) {
        None => return HttpResponse::build(StatusCode::BAD_REQUEST).finish(),
        Some(s) => match s.to_str() {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("convert analyzer header: {:?}", e);
                return HttpResponse::build(StatusCode::BAD_REQUEST).finish();
            }
        },
    };
    let hammer = match hammer.read() {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("ban hammer mutex {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let ban = match BanEntity::new(ban_req.0, anl.to_string()) {
        Ok(b) => b,
        Err(fe) => return fe.into(),
    };
    match hammer.ban(ban).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(skip(hammer))]
#[delete("/api/bans")]
async fn process_unban(
    unban_req: web::Json<UnBanRequest>,
    hammer: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
) -> impl Responder {
    let hammer = match hammer.read() {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("ban hammer mutex {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match hammer.unban(unban_req.0.target).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("ban target: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AdminRequest {
    dry_run: Option<bool>,
    log_level: Option<String>,
}

#[tracing::instrument(skip(bh))]
#[post("/api/admin")]
async fn admin_settings(
    q: web::Json<AdminRequest>,
    bh: Data<RwLock<Box<dyn BanHammerDryRunner + Sync + Send>>>,
    h: Data<Handle<EnvFilter, Formatter>>,
) -> impl Responder {
    let mut bh = match bh.write() {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("ban hammer mutex {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
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
