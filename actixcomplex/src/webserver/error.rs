//! This module is chaotic -> multiple methods for same things ( do not use )

use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, http, HttpResponse, Result};

// 400 Bad Request error handler
pub fn error_handler_400<B>(
    mut res: actix_web::dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    log::error!("Bad request - header{:?}", res.request());
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

pub async fn render_500<B>(
    mut res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

/// This method shows contents of `html/404.html` file
pub async fn error404<B>(
    req: actix_web::HttpRequest,
    //res: actix_web::dev::ServiceResponse<B>,
) -> Result<actix_http::Response, actix_web::Error> {
    log::info!("Default handler - some error");
    match actix_files::NamedFile::open("html/404.html") {
        Ok(file) => Ok(file
            .set_status_code(actix_web::http::StatusCode::NOT_FOUND)
            .into_response(&req)
            .unwrap()),
        Err(err) => Ok(HttpResponse::InternalServerError()
            .body(format!("Not possible to find 404.html - error: {}", err))),
    }
}

/// Method used for `actix_web::App::default_service`
pub mod default {
    /// 404 Not Found
    pub async fn error_404_file() -> actix_web::Result<actix_files::NamedFile> {
        log::info!("Default handler - some error");
        Ok(actix_files::NamedFile::open("html/404.html")?
            .set_status_code(actix_web::http::StatusCode::NOT_FOUND))
    }
}

// 400 Bad Request
pub async fn error400<B>(
    mut res: actix_web::dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    log::error!("Bad request - header{:?}", res.request());
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
