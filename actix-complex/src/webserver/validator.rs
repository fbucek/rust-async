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
pub fn auth_validator(
    req: dev::ServiceRequest,
    credentials: BasicAuth,
) -> Result<dev::ServiceRequest, Error> {
    match check_credentials(credentials) {
        Ok(_) => Ok(req),
        Err(err) => Err(err)
    }
}
