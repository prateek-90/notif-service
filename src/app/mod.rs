use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger as ActixLogger,
    web,
    web::Data,
    App, HttpRequest, HttpServer, Responder,
};
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
mod otp;
use super::appconfig::AppConfig;
use slog::*;
use slog_term::*;


#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub logger: Logger,
}

async fn index(_state: Data<AppState>, _req: HttpRequest) -> impl Responder {
    "Hello world!"
}

pub fn start(config: &AppConfig) {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let logger = Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!());

    info!(logger, "Logging ready!");
    let state = AppState {
        app_name: String::from("notif-worker"),
        logger: logger,
    };
    let bind_address = format!("127.0.0.1:{}", &config.server.port.to_string());

    let _cors = Cors::new()
        .allowed_origin("*")
        .send_wildcard()
        .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
        .max_age(3600);
    // let data = web::Data::new(Mutex::new(state));

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(ActixLogger::default())
            // .wrap(cors)
            .configure(routes)
    })
    // App::new().app_data(app_data.clone()).service(generate_otp).service(fetch_otp)
    // })
    .bind(&bind_address)
    .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
    .run();
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").to(index)).service(
        web::scope("/api").service(
            web::resource("/otp")
                .route(web::post().to(otp::generate_otp))
                .route(web::get().to(otp::fetch_otp)),
        ),
    );
}
