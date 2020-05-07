// Database
extern crate diesel;
// use diesel::prelude::*; // SqliteConneciton
// use diesel::r2d2;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{test, web, App};
use actix_web::dev::ServiceResponse;
use actix_service::Service;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");
//

struct RespString {
    status: actix_web::http::StatusCode,
    body: String,
}


/// Get method to test Actix server
/// ```ignore
/// let resp = get(&mut app, "/fake/users/id").await;
/// assert_eq!(resp.status.as_u16(), 404);
/// assert_eq!(resp.body, "");
/// ```
async fn get<S, B, E>(mut app: &mut S, url: &str) -> RespString
where
    B: actix_http::body::MessageBody,
    S: Service<Request = actix_http::Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let req = test::TestRequest::get().uri(url).to_request();
    let resp = test::call_service(&mut app, req).await;
    let status = resp.status().clone();
    let body = test::read_body(resp).await;
    let body = String::from_utf8_lossy(&body).to_string();

    RespString {
        status,
        body,
    }
}


mod integrations {
    //#[cfg_attr(test, macro_use)]
    use super::*;
    use actixjwt::api;

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
                .configure(api::users::config_app)
        )
        .await;


        let resp = get(&mut app, "/fake/users/id").await;
        assert_eq!(resp.status.as_u16(), 404);
        assert_eq!(resp.body, "");
        
        let resp = get(&mut app, "/users/1").await;
        assert_eq!(resp.status.as_u16(), 500); // user does not exists
        assert_eq!(resp.body, "");

        let resp = get(&mut app, "/users").await;
        assert_eq!(resp.status.as_u16(), 200);
        assert_eq!(resp.body, "[]");
    }
}