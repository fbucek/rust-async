use actix_web::{get, web, Responder};


#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
}

#[get("/password")]
async fn password(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
}


#[cfg(test)]
mod tests {

    //use futures::*;
    use super::*;

    #[actix_rt::test]
    async fn test_index_id_name() {
        std::env::set_var("RUST_LOG", "error,trace");

        let mut app = actix_web::test::init_service(
            actix_web::App::new()
            .service(index_id_name)
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
    }
}
