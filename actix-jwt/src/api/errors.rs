use actix_web::{error::ResponseError, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    JwtFetchError,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service Error")
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JwtFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
