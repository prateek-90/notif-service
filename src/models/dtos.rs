// pub mod connect_request;
use serde::{Serialize, Deserialize};
use futures::future::{ready, Ready};
// use crate::request::{HttpRequest, HttpResponse};
use actix_http::{Error, Response, ResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode};


#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectRequest {
    trigger_name: String,
    pub user_id: String,
    phone: String,
    config: String,
    product: String,
    email: String,
    params: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Otp {
    pub otp: Option<String>,
}

impl Responder for Otp {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}