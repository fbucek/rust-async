// <ssl>
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, error};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use lazy_static::lazy_static;
use std::collections::HashMap;

#[macro_use]
extern crate log;

lazy_static! {
    static ref ENDPOINTS: HashMap<&'static str, &'static str> = {
        let mut endpoints = HashMap::new();
        endpoints.insert("/v1/tablet/events", "calendar.ipsumlorem.net");
        endpoints.insert("/login/password","loremipsum.ipsumlorem.net");
        endpoints.insert("/api/v1/structure/rooms","loremipsum.ipsumlorem.net");
        endpoints.insert("/api/v2/support_reports","loremipsum.ipsumlorem.net");

        // Gitlab
        endpoints.insert("/gitlab-org/gitlab-foss/issues/62077","gitlab.com");
        // GitHub
        endpoints.insert("/users/octocat/orgs","api.github.com");

        endpoints
    };
}

/// @see https://github.com/actix/examples/blob/master/http-proxy/src/main.rs
pub async fn forward(
    req: HttpRequest,
    payload: web::Payload, // will be send to client
    // url: web::Data<Url>,
    client: web::Data<awc::Client>,
) -> Result<HttpResponse, actix_web::Error> {
    // Get host from endpoints hash
    let req_path = req.uri().path();
    let req_query = req.uri().query();

    // Get host to mapped endpoint
    // Internal server error when mapping not set
    // in: /v1/tablet/events
    // out: calendar.ipsumlorem.net
    let host = match ENDPOINTS.get(&req_path) {
        Some(host) => host,
        None => return Ok(HttpResponse::InternalServerError().body("not handled")),
    };

    // New url: https://calendar.ipsumlorem.net/v1/tablet/events
    let mut new_url: url::Url = format!("https://{}", &host).parse().unwrap();
    new_url.set_path(req_path);
    new_url.set_query(req_query);

    // info!("addr: )
    trace!("req: {:?}", req);

    // TODO: This forwarded implementation is incomplete as it only handles the inofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();

    // Change hosts
    let forwarded_req = forwarded_req.insert_header(("Connection", "keep-alive"));
    let forwarded_req = forwarded_req.insert_header(("host", *host));

    trace!("req: {:?}", forwarded_req);

    let res = forwarded_req
        .send_stream(payload)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut client_resp = HttpResponse::build(res.status());

    // This is needed for browser support
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.insert_header((header_name.clone(), header_value.clone()));
    }

    // Return client response with body
    Ok(client_resp.streaming(res))
}

async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome to https redirect proxy!"
}

/// load ssl keys
/// to create a self-signed temporary cert for testing:
/// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
///
/// # Add certificat to be trusted
///
/// https://apple.stackexchange.com/a/80625
/// `sudo security delete-certificate -c localhost`
/// `sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain localhost.crt`
///
/// `openssl req -x509 -out localhost.crt -keyout localhost.key \
///  -newkey rsa:2048 -nodes -sha256 \
///   -subj '/CN=localhost' -extensions EXT -config <( \
///    printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")`

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "trace, actixsslproxy=trace");
    env_logger::init();

    ////////////////////////////////////////
    // SSL setup
    let path = "ssl-keys/rustasync";
    let certificate = format!("{}.crt", &path);
    let private_key = format!("{}.key", &path);

    // let path = "ssl-keys/loremipsum-ipsumlorem-net";
    // let certificate = format!("{}.pem", &path);
    // let private_key = format!("{}.key", &path);

    let cur_dir = std::env::current_dir().expect("not possible to get current dir");
    println!("cur_dir: {}", &cur_dir.to_str().unwrap());
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&private_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&certificate).unwrap();

    println!("running on: https://localhost:8090");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .app_data(awc::Client::new())
            .default_service(web::route().to(forward))
    })
    .bind_openssl("localhost:8090", builder)?
    .run()
    .await?;

    Ok(())
}
