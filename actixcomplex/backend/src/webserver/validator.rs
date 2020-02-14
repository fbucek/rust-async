//! Basic http authentification
//!
//! ```rust
//! use actix_web::{App, get, HttpServer};
//! use actix_web_httpauth::middleware::HttpAuthentication;
//!
//! use actixcomplex::webserver;
//!
//! #[get("/")]
//! async fn index() -> &'static str {
//!     "Hello World!"
//! }
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!
//!     HttpServer::new(move || {
//!        let auth = HttpAuthentication::basic(webserver::validator::auth_validator);
//!        App::new()
//!            .wrap(auth)
//!            .service(index)
//!    })
//!    .bind("127.0.0.1:7070")?
//!    .run();
//!
//!     Ok(())
//! }
//!
//! ```

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
