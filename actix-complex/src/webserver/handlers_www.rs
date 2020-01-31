use actix_web::{get, web, Responder, HttpResponse};
use actix_web_httpauth::{
    extractors::basic::BasicAuth,
    middleware::HttpAuthentication,
};

use super::validator;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {

    let auth = HttpAuthentication::basic(validator::auth_validator);

    cfg
        .service(index)
        .service(index_id_name)
        .service(password)
        .service(web::scope("/public")
            .service(public_test)
        )
        .service(web::scope("/private")
            // .data(Config::default().realm("Restricted area"))
            .wrap(auth)
            .service(private_test)
            // .default_service(
            //     web::route().to(|| HttpResponse::Unauthorized().body("Not correct password or username")),
            // )
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
async fn password(auth: BasicAuth, info: web::Path<(u32, String)>) 
-> Result<actix_http::Response, actix_web::Error>  {
    trace!("First checking credentials");
    match validator::check_credentials(auth) {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("Hello {}! id:{}\n", info.1, info.0))),
        Err(_) => {
            trace!("unauthorized access");
            Ok(HttpResponse::Unauthorized().body("Wrong credentials"))
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

        let srv = actix_web::test::start(|| {
            actix_web::App::new()
                .configure(config)
        });

        let vec = vec![
            ("/", StatusCode::OK, "Hello World!"),
            ("", StatusCode::OK, "Hello World!"),
            ("/notfound", StatusCode::NOT_FOUND, ""),
            ("/34/filip/index.html", StatusCode::OK, "Hello filip! id:34\n"),
            ("/private/test", actix_web::http::StatusCode::UNAUTHORIZED, "Wrong username or password"),
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
