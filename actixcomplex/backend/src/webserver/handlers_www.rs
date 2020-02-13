use actix_web::{get, http, web, HttpResponse, Responder};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};

use super::validator;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    let auth = HttpAuthentication::basic(validator::auth_validator);

    cfg.service(index)
        .service(yew)
        .service(index_id_name)
        .service(actix_files::Files::new("/", "./actixcomplex/backend/static/"))
        .service(password)
        .service(web::scope("/public").service(public_test))
        .service(
            web::scope("/private")
                // .data(Config::default().realm("Restricted area"))
                .wrap(auth)
                .service(private_test), // .default_service(
                                        //     web::route().to(|| HttpResponse::Unauthorized().body("Not correct password or username")),
                                        // )
        );
}

#[get("/")]
async fn index() -> &'static str {
    "Hello World!"
}

#[get("/yew")]
async fn yew() -> Result<actix_http::Response, actix_web::Error> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/frontend.html")))
}

#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
}

#[get("/password/{id}/{name}")]
async fn password(
    auth: BasicAuth,
    info: web::Path<(u32, String)>,
    // @see https://docs.rs/actix-web/2.0.0/actix_web/trait.Responder.html
    // @see https://github.com/actix/actix-web/blob/6c9f9fff735023005a99bb3d17d3359bb46339c0/src/responder.rs#L106
    // ) -> impl Responder {
) -> Result<actix_http::Response, actix_web::Error> {
    trace!("First checking credentials");
    match validator::check_credentials(auth) {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("Hello {}! id:{}\n", info.1, info.0))),
        Err(err) => {
            debug!("unauthorized access");
            // Have to send OK with some data to notify user in browser ( sending error wont help )
            Ok(HttpResponse::Ok().body(format!("error: {:?}", err)))
        }
    }
}

#[get("/test")]
async fn public_test() -> &'static str {
    "Public!"
}

#[get("/test")]
async fn private_test() -> &'static str {
    "Private!"
}

#[cfg(test)]
mod tests {

    //use futures::*;
    use super::*;

    ///
    #[actix_rt::test]
    async fn test_index_id_name() {
        std::env::set_var("RUST_LOG", "error,trace");

        use actix_web::http::StatusCode;

        let srv = actix_web::test::start(|| actix_web::App::new().configure(config));

        let vec = vec![
            ("/", StatusCode::OK, "Hello World!"),
            ("", StatusCode::OK, "Hello World!"),
            ("/notfound", StatusCode::NOT_FOUND, ""),
            (
                "/34/filip/index.html",
                StatusCode::OK,
                "Hello filip! id:34\n",
            ),
            (
                "/private/test",
                actix_web::http::StatusCode::UNAUTHORIZED,
                "",
            ),
            ("/public/test", actix_web::http::StatusCode::OK, "Public!"),
        ];

        for test in vec {
            let uri = test.0;
            let status = test.1;
            let body = test.2;

            let mut response = srv.get(&uri).send().await.unwrap();
            assert_eq!(response.status(), status);
            if !body.is_empty() {
                let bytes = response.body().await.unwrap();
                assert_eq!(body, bytes);
            }
        }
    }
}
