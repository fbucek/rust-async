use actix_multipart::Multipart;
use actix_web::{get, middleware, post, App, Error, HttpResponse, HttpServer, Responder};
use async_std::prelude::*;
use futures_util::TryStreamExt as _;

#[macro_use]
extern crate log;

#[post("/api/v1/log/upload")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    trace!("Saving file");
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // let mut field = item?;
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .ok_or(actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            trace!("Got some chunk with size:{}", data.len());
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

#[get("/")]
async fn index() -> impl Responder {
    let html = r#"<html>
    <head><title>Upload Test</title></head>
    <body>
        <form target="/api/v1/log/upload" method="post" enctype="multipart/form-data">
            <input type="file" multiple name="file"/>
            <input type="submit" value="Submit"></button>
        </form>
    </body>
</html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "actixfileupload=trace,actix_server=info,actix_web=info",
    );
    async_std::fs::create_dir_all("./tmp").await?;

    env_logger::init();

    let ip = "localhost:3000";

    info!("Starting web server: {}", &ip);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(upload_file)
    })
    .bind(ip)?
    .run()
    .await
}
