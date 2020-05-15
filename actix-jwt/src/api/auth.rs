use crate::db::users::{InputUser, LoginRequest};
use crate::db::{self, Pool};
use actix_web::{post, web, Error, HttpRequest, HttpResponse, http::header::HeaderValue};

use serde::{Serialize, Deserialize};

use crate::utils::{token, response::ResponseBody};
use super::validator::*;
use crate::common;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth_validator);

    cfg.service(web::scope("api/auth")
        .service(signup_user)
        .service(login_user)
        //.wrap(auth)
        // .service(logout_user)
    );
    cfg.service(web::scope("api/private")
        // .wrap(auth)
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
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[post("/login")]
pub async fn login_user(
    db: web::Data<Pool>,
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db::users::login_user(db.into_inner(), &login_req.into_inner()))
            .await
            .map(|login_info| {
                let token_response = common::TokenBodyResponse {
                    token: token::UserToken::generate_token(login_info),
                    token_type: "bearer".to_string(),
                };
                HttpResponse::Ok().json(token_response)
            }) // status 200
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[post("/logout")]
pub async fn logout_user(
    // req: HttpRequest,
    // NOTE: Possible to have authorization header where but data must be set
    authorization: bearer::BearerAuth,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    info!("handling logout");
    let message = format!("Hello, user with token !"); //, authorization.token());
    // let message = format!("Hello, user with token {}!", authorization.token());

    // let token = authorization.token();
    let token = "test";

    // TODO: Needs to get `username`
    match logout(db, token).await {
        Ok(resp) => Ok(resp),
        Err(err) => Ok(HttpResponse::Ok().json(ResponseBody::new("Not possible to logout", "")))
    }
    // if let Some(authen_header) = req.headers().get("Authorization") {
    //     match logout(db, authen_header).await {
    //         Ok(resp) => Ok(resp),
    //         Err(err) => Ok(HttpResponse::Ok().json(ResponseBody::new("Not possible to logout", "")))
    //     }
    // } else {
    //     Ok(HttpResponse::BadRequest().json(ResponseBody::new("Token is missing", "")))
    // }
    // let username: &str = todo!("Have to get username from bearer header validation token");

    // if let Some(authen_header) = req.headers().get("Authorization") {




    // } else {
    //     Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    // }

    // if let Ok(authen_str) = authen_header.to_str() {
    //     if authen_str.starts_with("bearer") {
    //         let token = authen_str[6..authen_str.len()].trim();
    //         if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
    //             if let Ok(username) = token_utils::verify_token(&token_data, pool) {
    //                 if let Ok(user) = User::find_user_by_username(&username, &pool.get().unwrap()) {
    //                     User::logout(user.id, &pool.get().unwrap());
    //                     return Ok(());
    //                 }
    //             }
    //         }
    //     }
    // }



    // Ok(
    //     web::block(move || db::users::logout_user(db.into_inner(), &username))
    //         .await
    //         .map(|login_info| HttpResponse::Ok().json(login_info)) // status 200
    //         .map_err(|_| HttpResponse::InternalServerError())?,
    // )
}


async fn logout(pool: web::Data<Pool>, token: &str ) -> anyhow::Result<HttpResponse> {
    let pool = pool.into_inner();

    // let authen_str = token;
    // if ! authen_str.starts_with("bearer") {
    //     return Ok(HttpResponse::InternalServerError().json(
    //         ResponseBody::new("Not valid authorization header", "")
    //     ))
    // }

    // Decode username from token
    // let token = authen_str[6..authen_str.len()].trim();
    let token_data = jwt::decode_token(token)?;
    let username = jwt::verify_token(&token_data, pool.clone())?;

    Ok(
        web::block(move || db::users::logout_user(pool, &username))
            .await
            .map(|_| HttpResponse::Ok().json(ResponseBody::new("Login succesfull", "")))
            .map_err(|err| anyhow::anyhow!("Not possible to logout user: {:?}", err)
        )?
    )
}
