use actix_web::{get, http, web, HttpResponse, Responder};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};

use super::validator;

static HTML_LINKS: &str = "<a href='/yew'>yew</a><br>
<a href='/api/run'>api run</a><br>
<a href='password/41/filip'>password 41 filip</a><br>
<a href='/private/test'>private test</a>";

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    let auth = HttpAuthentication::basic(validator::auth_validator);

    cfg.service(index)
        .service(index_id_name)
        // .service(
        //     web::resource("/static/frontend.wasm")
        //         .route(web::get().to(|| {
        //             let file = actix_files::NamedFile::open("actixcomplex/backend/static/frontend.wasm");
        //             Ok(file)
        //                 // .header("content-encoding", "gzip")
        //                 // .content_encoding(http::header::ContentEncoding::Identity)
        //                 // .body(file)
        //         }
        //     )
        // )
        .service(password)
        .service(web::scope("/public").service(public_test))
        .service(
            web::scope("/private")
                // .data(Config::default().realm("Restricted area"))
                .wrap(auth)
                .service(private_test), // .default_service(
                                        //     web::route().to(|| HttpResponse::Unauthorized().body("Not correct password or username")),
                                        // )
        )
        // This must be last
        .service(yew)
        .service(actix_files::Files::new("/", "./actixcomplex/backend/static/").index_file("/yew"));
}

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(HTML_LINKS))
}

#[get("/yew")]
async fn yew() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/frontendyew.html")))
}

#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = info.into_inner();
    format!("Hello {}! id:{}\n", name, id)
}

#[get("/password/{id}/{name}")]
async fn password(
    auth: BasicAuth,
    path: web::Path<(u32, String)>,
    // @see <https://docs.rs/actix-web/2.0.0/actix_web/trait.Responder.html>
    // @see <https://github.com/actix/actix-web/blob/6c9f9fff735023005a99bb3d17d3359bb46339c0/src/responder.rs#L106>
    // ) -> impl Responder {
) -> Result<HttpResponse, actix_web::Error> {
    trace!("First checking credentials");
    let (id, name) = path.into_inner();
    match validator::check_credentials(auth) {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("Hello {}! id:{}\n", name, id))),
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
    use actix_http::body::MessageBody;
    use actix_web::test;

    ///
    #[actix_web::test]
    async fn test_index_id_name() {
        std::env::set_var("RUST_LOG", "error,trace");

        use actix_web::http::StatusCode;

        let app = test::init_service(actix_web::App::new().configure(config)).await;

        let vec = vec![
            ("/", StatusCode::OK, HTML_LINKS),
            // ("", StatusCode::OK, HTML_LINKS),
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
            let expected_body = test.2;

            let req = test::TestRequest::get().uri(uri).to_request();
            let response = test::call_service(&app, req).await;
            assert_eq!(response.status(), status);
            if !expected_body.is_empty() {
                let body = response.into_body();
                let body = body.try_into_bytes().unwrap();
                // let bytes = response.body().await.unwrap();
                assert_eq!(body, expected_body);
            }
        }
    }
}
