// <ssl>
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

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
    let certificate = "ssl-keys/rustasync.crt";
    let private_key = "ssl-keys/rustasync.key";

    let cur_dir = std::env::current_dir().expect("not possible to get current dir");
    println!("cur_dir: {}", &cur_dir.to_str().unwrap());
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&private_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&certificate).unwrap();

    println!("running on: https://localhost:8088");

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl("127.0.0.1:8088", builder)?
        .run()
        .await
}
