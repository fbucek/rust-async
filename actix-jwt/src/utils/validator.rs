use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};

//use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use crate::db::Pool;
use crate::utils::token;

// Reexport
pub use actix_web_httpauth::extractors::AuthenticationError;
pub use actix_web_httpauth::middleware::HttpAuthentication;
pub use actix_web_httpauth::extractors::bearer;


/// Actix custom validator implementation - Validates JSON Web Token
pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    trace!("auth_validator");
    // return Ok(req);
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);

    // FIXME: remove expect
    let pool = req.app_data::<Pool>().expect("not possible to get Pool");
    // .ok_or_else(|| Err(AuthenticationError::from(config).into()))?;

    // Now it is necessary to validate token form bearer authstring
    match token::UserToken::decode_token(credentials.token()) {
        Ok(token_data) => {
            info!("Decoding token succesfull");
            if token::jwt::verify_token(&token_data, pool.into_inner()).is_ok() {
                info!("Valid token");
                Ok(req)
            } else {
                error!("Invalid token");
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
