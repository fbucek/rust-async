use crate::db::users::{InputUser, LoginRequest};
use crate::db::{self, Pool};
use actix_web::{post, web, Error, HttpResponse};

use crate::utils::{errors::ServiceError, token, response::ResponseBody, validator::*};
use crate::common;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth_validator);

    cfg.service(web::scope("api/users")
        .service(signup_user)
        .service(login_user)
        //.wrap(auth)
        // .service(logout_user)
    );
    cfg.service(web::scope("api/auth")
        .wrap(auth)
        .service(logout_user)
    );
}

#[post("/signup")]
pub async fn signup_user(
    db: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db::users::signup_user(db.into_inner(), &user.into_inner()))
            .await
            .map(|user| HttpResponse::Created().json(user)) // status 201
            .map_err(|err| ServiceError::LoginError(format!("Not possible to login user: {:?}", err)))?,
    )
}

#[post("/login")]
pub async fn login_user(
    db: web::Data<Pool>,
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    info!("User: {} trying to login", login_req.username);

    let result = web::block(move || db::users::login_user(db.into_inner(), &login_req.into_inner()))
            .await;
    
    match result {
        Ok(login_info) => {
            match token::UserToken::generate_token(login_info) {
                Ok(token) => {
                    let token_response = common::TokenBodyResponse {
                        token,
                        token_type: "bearer".to_string(),
                    };
                    Ok(HttpResponse::Ok().json(token_response))
                }
                Err(err) => Err(ServiceError::LoginError(format!("Not possible to generate token: {:?}", err)))?,
            }
        }
        Err(err) => {
            Err(ServiceError::LoginError(format!("Not possible to login user: {:?}", err)))?
        }
    }
}

#[post("/logout")]
pub async fn logout_user(
    // req: HttpRequest,
    // NOTE: Possible to have authorization header where but data must be set
    authorization: bearer::BearerAuth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    // Bearer token
    let token = authorization.token();

    match logout(pool, token).await {
        Ok(resp) => Ok(resp),
        Err(err) => Ok(HttpResponse::Ok()
            .json(ResponseBody::new(&format!("Not possible to logout: {:?}", err), "")))
    }
}


async fn logout(pool: web::Data<Pool>, token: &str ) -> anyhow::Result<HttpResponse> {
    let pool = pool.into_inner();

    // Decode username from token
    // let token = authen_str[6..authen_str.len()].trim();
    let token_data = token::UserToken::decode_token(token)?;
    let username = token::jwt::verify_token(&token_data, pool.clone())?;

    Ok(
        web::block(move || db::users::logout_user(pool, &username))
            .await
            .map(|_| HttpResponse::Ok().json(ResponseBody::new("Logout succesfull", "")))
            .map_err(|err| anyhow::anyhow!("Not possible to logout user: {:?}", err)
        )?
    )
}
