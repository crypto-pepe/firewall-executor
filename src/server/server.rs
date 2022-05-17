use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{
    dev, error, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder,
};
use mime;
use tokio::io;
use tracing_actix_web::TracingLogger;

use crate::ban_hammer::redis::RedisBanHammer;
use crate::ban_hammer::BanHammer;
use crate::model::{BanEntity, BanRequest};
use crate::server::Config;
use crate::ANALYZER_HEADER;

pub struct Server {
    srv: dev::Server,
}

impl Server {
    pub fn new(cfg: &Config, bh: RedisBanHammer) -> Result<Server, io::Error> {
        let bh = Data::from(Arc::new(bh));

        let srv = HttpServer::new(move || {
            App::new()
                .app_data(bh.clone())
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
        cfg.app_data(json_cfg).service(process_ban);
    })
}

#[tracing::instrument(skip(req, hammer))]
#[post("/api/bans")]
async fn process_ban(
    req: actix_web::HttpRequest,
    ban_req: web::Json<BanRequest>,
    hammer: Data<RedisBanHammer>,
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
