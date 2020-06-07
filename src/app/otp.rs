use crate::models::{ConnectRequest, Otp};
use actix_web::{ web, web::Json, App, HttpServer, Responder, HttpResponse, http::StatusCode};
use super::AppState;
use actix_http::body::{Body, BodySize, MessageBody, ResponseBody, SizedStream};
// info: web::Path<i32>, data: web::Data<AppState>
pub async fn fetch_otp(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    println!("Fetch otp from app {:?}", app_name);
    let otp = Otp {otp: Some(String::from("121345")),};
    serde_json::to_string(&otp).unwrap_or_else(|err| format!("No otp found"))
}

pub async fn generate_otp(mut payload: web::Json<ConnectRequest>, data: web::Data<AppState>) -> HttpResponse {
    let app_name = &data.app_name; // <- get app_name
    println!("Fetch otp from app {:?}", app_name);
    // let connect_req:  = serde_json::from_value(serde_json::Value::String(payload)).unwrap();
    println!("Hello connect req {:?}", payload);
    HttpResponse::new(StatusCode::OK)
    // format!("Request to generate otp for user {}", payload.user_id)
}