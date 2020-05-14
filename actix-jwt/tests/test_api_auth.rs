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
    use actixjwt::db::users::{InputUser, UserInfo};

    #[actix_rt::test]
    async fn test_auth() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = &test_db
            .conn()
            .expect("Not possible to get pooled connection");
        embedded_migrations::run(conn).expect("Migration not possible to run");

        let mut app = test::init_service(
            App::new()
                .data(test_db.pool)
                .configure(api::users::config_app)
                .configure(api::auth::config_app),
        )
        .await;

        let user = InputUser {
            username: "johndoe".to_string(),
            password: "strong xxx".to_string(),
            email: "johndoe@apple.com".to_string(),
        };

        // POST User
        let resp = testax::post_json(&mut app, &user, "/users").await;
        assert_eq!(resp.status.as_u16(), 201);
        let dbuser: UserInfo = serde_json::from_str(&resp.body).unwrap();
        assert_eq!(dbuser.username, user.username);

        // POST Login
        let resp = testax::post_json(&mut app, &user, "/api/auth/login").await;
        assert_eq!(resp.status.as_u16(), 200);
        //let json = serde_json::json!()
        // TODO:
        // assert_eq!(resp.body, "");
    }
}
