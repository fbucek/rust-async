use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

//use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use crate::db::{users, Pool};
use std::sync::Arc;

pub mod jwt {
    use super::*;
    use crate::utils::token::{UserToken, KEY};
    use jsonwebtoken::{DecodingKey, TokenData, Validation};

    pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
        jsonwebtoken::decode::<UserToken>(
            token,
            &DecodingKey::from_secret(&KEY),
            &Validation::default(),
        )
    }

    pub fn verify_token(
        token_data: &TokenData<UserToken>,
        pool: Arc<Pool>,
    ) -> Result<String, String> {
        let username = &token_data.claims.user;
        let login_session = &token_data.claims.login_session;
        //

        //
        if users::is_valid_login_session(pool, username, login_session) {
            Ok(token_data.claims.user.to_string())
        } else {
            Err("Invalid token".to_string())
        }
    }
} // mod jwt

/// Actix custom validator implementation - Validates JSON Web Token
async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);

    // FIXME: remove expect
    let pool = req.app_data::<Pool>().expect("not possible to get Pool");
    // .ok_or_else(|| Err(AuthenticationError::from(config).into()))?;

    // Now it is necessary to validate token form bearer authstring
    match jwt::decode_token(credentials.token()) {
        Ok(token_data) => {
            info!("Decoding token succesfull");
            if jwt::verify_token(&token_data, pool.into_inner()).is_ok() {
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
