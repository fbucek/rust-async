// Database
extern crate diesel;
// use diesel::prelude::*; // SqliteConneciton
// use diesel::r2d2;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{test, web, App, Error};
use actix_web::dev::{ServiceResponse};

// use actix_http::http::header::{ContentType, Header, HeaderName, IntoHeaderValue};
// use actix_http::http::{Error as HttpError, Method, StatusCode, Uri, Version};
// use actix_http::test::TestRequest as HttpTestRequest;
// use actix_http::{cookie::Cookie, ws, Extensions, HttpService, Request};
// use actix_router::{Path, ResourceDef, Url};
use actix_rt::{time::delay_for, System};
use actix_service::{
    map_config, IntoService, IntoServiceFactory, Service, ServiceFactory,
};


// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!("migrations");
//

struct Resp {
    status: bool,
    body: Option<String>,
}

pub async fn call_service_orig<S, R, B, E>(app: &mut S, req: R) -> S::Response
where
    S: Service<Request = R, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    app.call(req).await.unwrap()
}

pub async fn call_service<S, B, E>(app: &mut S) -> S::Response
where
    S: Service<Request = actix_http::Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let req = test::TestRequest::get().uri("/users/id").to_request();
    app.call(req).await.unwrap()
}


// pub async fn get<S, B, E, U>(app: &mut S, url: U) -> Resp
// where
// S: Service<Response = ServiceResponse<B>, Error = E>,
// E: std::fmt::Debug,
// U: Into<String>,
// {
//      // /users/id must pass
//      let req = test::TestRequest::get().uri("/users/id").to_request();
//      let resp = test::call_service(&mut app, req).await;

//      assert!(resp.status().is_success());
//      let body = match resp.response().body().as_ref() {
//          Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//          _ => panic!("Response error"),
//      };


//     // Response {
//     //     status,
//     //     body,
//     // }
// }




mod integrations {
    //#[cfg_attr(test, macro_use)]
    use super::*;
    use actixjwt::api;

    #[actix_rt::test]
    async fn test_get_user() -> Result<(), Error> {
        // TODO addd database test

        let test_db = nafta::sqlite::TestDb::new();
        let conn = &test_db
            .conn()
            .expect("Not possible to get pooled connection");
        embedded_migrations::run(conn).expect("Migration not possible to run");

        let mut app = test::init_service(
            App::new()
                .data(test_db.pool)
                .service(web::resource("/users/id").route(web::get().to(api::users::get_users))),
        )
        .await;

        
        // FAIL: /fake/users/id
        let req = test::TestRequest::get().uri("/fake/users/id").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(!resp.status().is_success());

        // /users/id must pass
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
