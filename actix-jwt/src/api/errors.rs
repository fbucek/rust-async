use actix_web::{error::ResponseError, HttpResponse};

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    JwtFetchError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JwtFetchError => {
                HttpReponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
