use crate::models::{ConnectRequest, Otp};
use actix_web::{ web, web::Json, App, HttpServer, Responder, HttpResponse, http::StatusCode};
use super::AppState;
use actix_http::body::{Body, BodySize, MessageBody, ResponseBody, SizedStream};
use slog::{Drain, o, info};
use crate::utils;

pub async fn fetch_otp(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    info!(&data.logger, "fetch from app {}",  app_name);
    let otp = utils::generate_randop_otp(&data.logger);
    let otp = Otp {otp: Some(otp),};
    serde_json::to_string(&otp).unwrap_or_else(|err| format!("No otp found"))
}

pub async fn generate_otp(mut payload: web::Json<ConnectRequest>, data: web::Data<AppState>) -> HttpResponse {
    let app_name = &data.app_name; // <- get app_name
    info!(&data.logger, "fetch from app {}, connect req {:?}",  app_name, payload);
    HttpResponse::new(StatusCode::OK)
}