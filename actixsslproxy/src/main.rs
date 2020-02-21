// <ssl>
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[macro_use]
extern crate log;

/// @see https://github.com/actix/examples/blob/master/http-proxy/src/main.rs
pub async fn forward(
    req: HttpRequest,
    body: web::Bytes, // will be send to client
    // url: web::Data<Url>,
    client: web::Data<awc::Client>,
) -> Result<actix_http::Response, actix_web::Error> {
    // 
    let host = "api.github.com";
    let url: url::Url = format!("https://{}", &host).parse().unwrap();
    let mut new_url = url.clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    // info!("addr: )
    trace!("req: {:?}", req);

    // TODO: This forwarded implementation is incomplete as it only handles the inofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();

    // Change hosts
    let forwarded_req = forwarded_req.set_header("Connection", "keep-alive");
    let forwarded_req = forwarded_req.set_header("host", host);

    trace!("req: {:?}", forwarded_req);

    let mut res = forwarded_req.send_body(body).await.map_err(actix_web::Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());

    Ok(client_resp.body(res.body().await?))
}


async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actixsslproxy=trace");
    env_logger::init();

    // let path = "ssl-keys/rustasync";
    // let certificate = format!("{}.crt", &path);
    // let private_key = format!("{}.key", &path);
    
    let path = "ssl-keys/loremipsum-ipsumlorem-net";
    let certificate = format!("{}.pem", &path);
    let private_key = format!("{}.key", &path);

    let cur_dir = std::env::current_dir().expect("not possible to get current dir");
    println!("cur_dir: {}", &cur_dir.to_str().unwrap());
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&private_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&certificate).unwrap();

    println!("running on: https://localhost:8090");

    HttpServer::new(|| { 
        App::new().route("/", web::get().to(index))
            .data(awc::Client::new())
            .default_service(web::route().to(forward))
    })
        .bind_openssl("127.0.0.1:8090", builder)?
        .workers(10)
        .run()
        .await
}
