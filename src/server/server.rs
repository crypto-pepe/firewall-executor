use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{
    dev, error, http::StatusCode, middleware::Logger, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use mime;
use pepe_log::error;
use serde_json::json;
use tokio::io;

use crate::ban_checker::BanChecker;
use crate::ban_hammer::BanHammer;
use crate::model::{BanEntity, BanRequest, BanTargetRequest};
use crate::redis::Service;
use crate::server::Config;

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(cfg: &Config, bh: Service) -> Result<Server, io::Error> {
        let bh = Data::from(Arc::new(bh));

        let json_cfg = web::JsonConfig::default()
            .content_type(|mime| mime == mime::APPLICATION_JSON)
            .error_handler(|err, _| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().into()).into()
            });

        let srv = HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(bh.clone())
                .app_data(json_cfg.clone())
                .service(process_ban)
                .service(check_ban)
        })
        .bind((cfg.host.clone(), cfg.port))?
        .run();
        Ok(Server { srv })
    }

    pub async fn run(self) -> io::Result<()> {
        self.srv.await
    }
}

#[post("/api/ban")]
async fn process_ban(
    req: actix_web::HttpRequest,
    ban_req: web::Json<BanRequest>,
    hammer: Data<Service>,
) -> impl Responder {
    let anl = match req.headers().get("X-Analyzer-Id") {
        None => return HttpResponse::build(StatusCode::BAD_REQUEST).finish(),
        Some(s) => s.to_str().unwrap().to_string(),
    };
    let ban = match BanEntity::from(ban_req.0, anl.clone()) {
        Ok(b) => b,
        Err(fe) => return fe.into(),
    };
    match hammer.ban(ban).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/check-ban")]
async fn check_ban(ban_req: web::Json<BanTargetRequest>, checker: Data<Service>) -> impl Responder {
    match checker.check(&ban_req.target).await {
        Ok(o) => match o {
            None => HttpResponse::Ok().json(json!({"status":"free"})),
            Some(ttl) => HttpResponse::Ok().json(json!({"status":"banned", "ban_expires_at":ttl})),
        },
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
