use actix_web::{error::ResponseError, HttpResponse};
use super::response::ResponseBody;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    // #[display(fmt = "Internal Server Error")]
    InternalServerError,
    // #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),
    DbError(String),
    JWTError(String),
    LoginError(String),
}

/// This can be done automatically using `derive_more` -> #[derive(Display)]
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::InternalServerError => write!(f, "Internal Server Errorxxx"),
            ServiceError::BadRequest(message) => write!(f, "BadRequest: {}", message),
            ServiceError::DbError(message) => write!(f, "DB erorr: {}", message),            
            ServiceError::JWTError(message) => write!(f, "JWT validation failed: {}", message),            
            ServiceError::LoginError(message) => write!(f, "Login failed: {}", message),            
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWTError(message) => 
                HttpResponse::InternalServerError().json(ResponseBody::new(message, "")),
            ServiceError::DbError(message) => 
                HttpResponse::InternalServerError().json(ResponseBody::new(message, "")),
            ServiceError::LoginError(message) => 
                HttpResponse::Unauthorized().json(ResponseBody::new(message, "")),
        }
    }
}
