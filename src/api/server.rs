use actix_web::web::Data;
use actix_web::{dev, error, web, App, HttpResponse, HttpServer, ResponseError};
use mime;
use std::sync::Arc;
use tokio::io;
use tokio::sync::RwLock;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::Formatter;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::EnvFilter;

use crate::api::{routes, Config};
use crate::api::http_error::ErrorResponse;
use crate::ban_hammer::BanHammerDryRunner;

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
                let reason = err.to_string();
                error::InternalError::from_response(err, ErrorResponse{
                    code: 400,
                    reason,
                    details: None
                }.error_response()).into()
            });
        cfg.app_data(json_cfg)
            .service(routes::process_ban)
            .service(routes::configuration_handler)
            .service(routes::process_unban);
    })
}
