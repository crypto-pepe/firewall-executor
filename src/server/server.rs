use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{
    dev, error, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder,
};
use mime;
use pepe_log::error;
use tokio::io;

use crate::ban_hammer::redis::RedisBanHammer;
use crate::ban_hammer::BanHammer;
use crate::model::{BanEntity, BanRequest};
use crate::server::Config;

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(cfg: &Config, bh: RedisBanHammer) -> Result<Server, io::Error> {
        let bh = Data::from(Arc::new(bh));

        let srv =
            HttpServer::new(move || App::new().app_data(bh.clone()).configure(server_config()));

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
        cfg.app_data(json_cfg).service(process_ban);
    })
}

#[post("/api/bans")]
async fn process_ban(
    req: actix_web::HttpRequest,
    ban_req: web::Json<BanRequest>,
    hammer: Data<RedisBanHammer>,
) -> impl Responder {
    let anl = match req.headers().get("X-Analyzer-Id") {
        None => return HttpResponse::build(StatusCode::BAD_REQUEST).finish(),
        Some(s) => s.to_str().unwrap().to_string(),
    };
    let ban = match BanEntity::new(ban_req.0, anl.clone()) {
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
