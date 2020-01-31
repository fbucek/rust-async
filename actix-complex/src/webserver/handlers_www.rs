use actix_web::{get, web, Responder};
use actix_web_httpauth::{
    extractors::basic::{BasicAuth, Config},
    middleware::HttpAuthentication,
};

use super::validator;

pub fn config(_cfg: &mut actix_web::web::ServiceConfig) {

    let auth = HttpAuthentication::basic(validator::auth_validator);

    _cfg
        .service(index)
        .service(index_id_name)
        .service(password)
        .service(web::scope("/public")
            .service(public_test)
        )
        .service(web::scope("/private")
            .data(Config::default().realm("Restricted area"))
            .wrap(auth)
            .service(private_test)
        );
}

#[get("/")]
async fn index() -> &'static str {
    "Hello World!"
}


#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
}

#[get("/password/{id}/{name}")]
async fn password(auth: BasicAuth, info: web::Path<(u32, String)>) -> impl Responder {
    match validator::check_credentials(auth) {
        Ok(_) => Ok(format!("Hello {}! id:{}\n", info.1, info.0)),
        Err(err) => Err (err)
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

        let srv = actix_web::test::start(|| {
            actix_web::App::new()
                .configure(config)
        });

        let vec = vec![
            ("/34/filip/index.html", StatusCode::UNAUTHORIZED, "Hello filip! id:34\n"),
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

    /// Service test ( not necessary )
    #[actix_rt::test]
    async fn test_index_id_name_service() {
        std::env::set_var("RUST_LOG", "error,trace");

        let mut app = actix_web::test::init_service(
            actix_web::App::new()
                .configure(config)
        ).await;

        let in_uri = "/34/filip/index.html";
        let out_body = "Hello filip! id:34\n";

        let server_request = actix_web::test::TestRequest::with_uri(&in_uri).to_request();
        let server_response = actix_web::test::call_service(&mut app, server_request).await;
        // Check status
        assert_eq!(server_response.status(), actix_web::http::StatusCode::OK);
        assert!(server_response.status().is_success());
        // Check body
        let body = actix_web::test::read_body(server_response).await;
        assert_eq!(body, &out_body);


        // No path -> NOT_FOUND 404
        let server_request = actix_web::test::TestRequest::with_uri("/35/filip/").to_request();
        let server_response = actix_web::test::call_service(&mut app, server_request).await;
        // Check status
        assert_eq!(server_response.status(), actix_web::http::StatusCode::NOT_FOUND);
    }




    /// 
    #[actix_rt::test]
    async fn test_auth() {
        std::env::set_var("RUST_LOG", "error,trace");

        let srv = actix_web::test::start(|| {
            actix_web::App::new()
                .configure(config)
        });
        
        let vec = vec![
            ("/private/test", actix_web::http::StatusCode::UNAUTHORIZED),
            ("/public/test", actix_web::http::StatusCode::OK),
        ];

        for test in vec {
            let uri = test.0;
            let status = test.1;
            let response = srv.get(&uri).send().await.unwrap();
            assert_eq!(response.status(), status);
        }
    }
}
