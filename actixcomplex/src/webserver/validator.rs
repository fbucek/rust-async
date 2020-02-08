//! Basic http authentification
//! 
//! ```
//! use actix_web::{App, HttpServer};
//! use futures::lock::Mutex;
//! use std::sync::Arc;
//! use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//! 
//! use std::*;
//! 
//! #[macro_use]
//! extern crate log;
//! 
//! use actixcomplex::controller;
//! use actixcomplex::webserver;
//! 
//! #[actix_rt::main]
//! async fn main() -> io::Result<()> {
//!     let auth = HttpAuthentication::basic(validator::auth_validator);
//! 
//! //    HttpServer::new(move || {
//! //       App::new()
//! //           .wrap()
//! //           .configure(webserver::handlers_api::config)
//! //           .configure(webserver::handlers_www::config)
//! //           .data(Arc::clone(&sender))
//! //   })
//! //   .bind("127.0.0.1:8080")?
//! //   .run();
//!
//!     Ok(())
//! }
//! 
//! 
//! 
//! #[actix_rt::main]
//! ```
//! 
//! 
// Actix



use actix_web::{dev, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;

/// Check if user and password is correct
pub fn check_credentials(credentials: BasicAuth) -> Result<(), Error> {
    let password = match credentials.password() {
        Some(password) => password,
        None => "",
    };

    if credentials.user_id() == "admin" && password == "password" {
        Ok(())
    } else {
        Err(actix_web::error::ErrorUnauthorized(
            "Wrong username or password",
        ))
    }
}

/// Middleware validator
pub async fn auth_validator(
    req: dev::ServiceRequest,
    credentials: BasicAuth,
) -> Result<dev::ServiceRequest, Error> {
    trace!("auth_validator checking");
    match check_credentials(credentials) {
        Ok(_) => Ok(req),
        Err(err) => {
            trace!("Wrong credentials");
            Err(err)
        }
    }
}
