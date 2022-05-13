use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{
    dev, error, http::StatusCode, middleware::Logger, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use mime;
use pepe_log::error;
use serde::{Deserialize, Serialize};
use slog_extlog_derive::SlogValue;
use thiserror::Error;
use tokio::io;

use crate::ban_hammer::redis_impl::RedisService;
use crate::ban_hammer::BanHammer;
use crate::model::BanEntity;
use crate::BanRequest;

#[derive(Clone, Debug, Serialize, Deserialize, SlogValue)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    RunError(#[from] io::Error),
}

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(cfg: &Config, bh: RedisService) -> Result<Server, io::Error> {
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
    hammer: Data<RedisService>,
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
