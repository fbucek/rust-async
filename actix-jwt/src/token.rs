//! Everything related to web token handling

use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use uuid::Uuid;

mod test;

const SECRET: &[u8] = b"my_secret";

#[derive(Debug, Error)]
/// Token handling related errors
pub enum TokenError {
    #[error("unable to create session token")]
    /// Session token creation failed
    Create,

    #[error("unable to verify session token")]
    /// Session token verification failed
    Verify,
}

impl ResponseError for TokenError {
    fn error_response(&self) -> HttpResponse {
        match self {
            TokenError::Create => HttpResponse::InternalServerError().into(),
            TokenError::Verify => HttpResponse::Unauthorized().into(),
        }
    }
}

#[derive(Deserialize, Serialize)]
/// A web token
pub struct Token {
    /// The subject of the token
    payload: String,

    /// The exipration date of the token
    expiration_date: i64,

    /// Token created at
    valid_from: i64,

    /// The token id
    token_id: String,
}

impl Token {
    /// Create a new default token for a given username
    pub fn create(username: &str) -> Result<String, TokenError> {
        const DEFAULT_TOKEN_VALIDITY: i64 = 3600;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TokenError::Create)?;
        let claim = Token {
            payload: username.to_owned(),
            expiration_date: now.as_secs() as i64 + DEFAULT_TOKEN_VALIDITY,
            valid_from: now.as_secs() as i64,
            token_id: Uuid::new_v4().to_string(),
        };
        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(SECRET),
        )
        .map_err(|_| TokenError::Create)
    }

    /// Verify the validity of a token and get a new one
    pub fn verify(token: &str) -> Result<String, TokenError> {
        let data = decode::<Token>(
            token,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )
        .map_err(|_| TokenError::Verify)?;
        Self::create(&data.claims.payload)
    }
}
