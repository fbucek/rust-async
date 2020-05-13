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

mod integrations {
    //#[cfg_attr(test, macro_use)]
    use super::*;
    use actixjwt::api;
    use actixjwt::db::users::{InputUser, User};

    #[actix_rt::test]
    async fn test_get_user() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = &test_db
            .conn()
            .expect("Not possible to get pooled connection");
        embedded_migrations::run(conn).expect("Migration not possible to run");

        let mut app = test::init_service(
            App::new()
                .data(test_db.pool)
                .configure(api::users::config_app),
        )
        .await;

        let resp = testax::get(&mut app, "/fake/users/id").await;
        assert_eq!(resp.status.as_u16(), 404);
        assert_eq!(resp.body, "");

        let resp = testax::get(&mut app, "/users/1").await;
        assert_eq!(resp.status.as_u16(), 500); // user does not exists
        assert_eq!(resp.body, "");

        let resp = testax::get(&mut app, "/users").await;
        assert_eq!(resp.status.as_u16(), 200);
        assert_eq!(resp.body, "[]");
    }

    #[actix_rt::test]
    async fn test_new_user() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = &test_db
            .conn()
            .expect("Not possible to get pooled connection");
        embedded_migrations::run(conn).expect("Migration not possible to run");

        let mut app = test::init_service(
            App::new()
                .data(test_db.pool)
                .configure(api::users::config_app),
        )
        .await;

        let user = InputUser {
            username: "johndoe".to_string(),
            password: "strong xxx".to_string(),
            email: "johndoe@apple.com".to_string(),
        };

        let resp = testax::post_json(&mut app, &user, "/users").await;
        assert_eq!(resp.status.as_u16(), 201);
        let dbuser: User = serde_json::from_str(&resp.body).unwrap();
        assert_eq!(dbuser.username, user.username);

        let resp = testax::get(&mut app, "/users/1").await;
        assert_eq!(resp.status.as_u16(), 200); // user does not exists
        let dbuser: User = serde_json::from_str(&resp.body).unwrap();
        assert_eq!(dbuser.username, user.username);
        assert_eq!(dbuser.id, 1);

        let resp = testax::get(&mut app, "/users").await;
        assert_eq!(resp.status.as_u16(), 200);
        let dbusers: Vec<User> = serde_json::from_str(&resp.body).unwrap();
        assert_eq!(dbusers.len(), 1);
        assert_eq!(dbusers.first().unwrap(), &dbuser);
    }

    #[actix_rt::test]
    async fn test_fail() {
        let test_db = nafta::sqlite::TestDb::new();
        let conn = &test_db
            .conn()
            .expect("Not possible to get pooled connection");
        embedded_migrations::run(conn).expect("Migration not possible to run");

        let mut app = test::init_service(
            App::new()
                .data(test_db.pool)
                .configure(api::users::config_app),
        )
        .await;

        let user = serde_json::json!({
            "not": "full_name"
        });

        // BAD REQUEST
        let resp = testax::post_json(&mut app, &user, "/users").await;
        assert_eq!(resp.status.as_u16(), 400);
        assert_eq!(resp.body, "");
        
        // INTERNAL SERVER ERROR
        let resp = testax::get(&mut app, "/users/1").await;
        assert_eq!(resp.status.as_u16(), 500); // user does not exists
        assert_eq!(resp.body, "");

        // EMPTY RESPONSE
        let resp = testax::get(&mut app, "/users").await;
        assert_eq!(resp.status.as_u16(), 200);
        let dbusers: Vec<User> = serde_json::from_str(&resp.body).unwrap();
        assert_eq!(dbusers.len(), 0);
    }
}
