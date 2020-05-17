use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::users::LoginInfo;
use crate::db::users;
use crate::db::Pool;

pub static KEY: [u8; 16] = *include_bytes!("../../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds


#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}


#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(login: LoginInfo) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.username,
            login_session: login.login_session,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        ).expect("Not possible to encode JWT token")
    }
}

/// Json Web Token / validation https://jwt.io/
pub mod jwt {
    use super::*;
    use crate::utils::token::{UserToken, KEY};
    use jsonwebtoken::{DecodingKey, TokenData, Validation};

    /// Decode JWT from 'str' into JWD data  
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
    ) -> anyhow::Result<String> {
        let username = &token_data.claims.user;
        let login_session = &token_data.claims.login_session;
        //

        //
        if users::is_valid_login_session(pool, username, login_session) {
            Ok(token_data.claims.user.to_string())
        } else {
            Err(anyhow::anyhow!("Invalid token"))
        }
    }
} // mod jwt
