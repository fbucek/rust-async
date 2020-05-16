// Database
extern crate diesel;
// use diesel::prelude::*; // SqliteConneciton
// use diesel::r2d2;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{test, App};

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");
//

mod api_auth {
    //#[cfg_attr(test, macro_use)]
    use super::*;
    use actixjwt::api;
    use actixjwt::db::users::{InputUser, UserInfo, LoginRequest, LoginInfo};
    use actixjwt::common;
    use actixjwt::api::validator::*;

    fn create_user(user: &str, pass: &str, email: &str) -> InputUser {
        InputUser {
            username: user.to_string(),
            password: pass.to_string(),
            email: email.to_string(),
        }
    }

    fn create_login_request(user: &str, password: &str) -> LoginRequest {
        LoginRequest {
            username: user.to_string(),
            password: user.to_string(),
        }
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[actix_rt::test]
    async fn test_auth() {
        // init();
        // log::info!("Running test");
        env_logger::init();
        let test_db = nafta::sqlite::TestDb::new();
        let conn = &test_db
            .conn()
            .expect("Not possible to get pooled connection");
        embedded_migrations::run(conn).expect("Migration not possible to run");


        let auth = HttpAuthentication::bearer(auth_validator);

        let mut app = test::init_service(

            App::new()
                // .wrap(auth)
                .data(test_db.pool)
                .configure(api::users::config_app)
                .configure(api::auth::config_app),
        ).await;

        let username = "johndoe";
        let password = "strong xxx";
        let user = create_user(username,password, "johndoe@apple.com");
        
        // POST User
        let resp = testax::post_json(&mut app, &user, "/api/auth/signup").await;
        assert_eq!(resp.status.as_u16(), 201);
        let dbuser: UserInfo = serde_json::from_str(&resp.body).unwrap();
        assert_eq!(dbuser.username, user.username);
        
        // POST Login
        let login_req = create_login_request(username, password);
        let resp = testax::post_json(&mut app, &login_req, "/api/auth/login").await;
        assert_eq!(resp.status.as_u16(), 200);
        let json_body : common::TokenBodyResponse = serde_json::from_str(&resp.body)
            .expect("Not possible to parse TokenBodyResponse token from body");
        log::trace!("token body: {:?}", json_body);   
        
        let req = test::TestRequest::post()
            .header("Authorization", format!("Bearer {}", json_body.token))
            .uri("/api/private/logout").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 200);
        let body = test::read_body(resp).await;
        let body = String::from_utf8_lossy(&body).to_string();
        assert_eq!(body, "d");
    }
}
