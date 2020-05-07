
// Database
extern crate diesel;
// use diesel::prelude::*; // SqliteConneciton
// use diesel::r2d2;

#[macro_use]
extern crate diesel_migrations;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");
//


mod integrations {
    //#[cfg_attr(test, macro_use)]
    use super::*;
    use actix_web::{App, Error, test, web};
    use actixjwt::db::schema::users::{self, dsl::*};
    use actixjwt::api;

    #[actix_rt::test]
    async fn test_get_user() -> Result<(), Error> {
        // TODO addd database test

        let test_db = nafta::sqlite::TestDb::new();
        let conn = &test_db.conn().expect("Not possible to get pooled connection");
        embedded_migrations::run(conn).expect("Migration not possible to run");

        let mut app = test::init_service(
            App::new()
                .data(test_db.pool)
                .service(web::resource("/users/id").route(web::get().to(api::users::get_users))),
        )
        .await;

        let req = test::TestRequest::get().uri("/users/id").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, "[]");

        Ok(())
    }
}
