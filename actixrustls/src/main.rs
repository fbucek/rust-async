use std::fs::File;
use std::io::BufReader;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let certificate = "ssl-keys/rustasync.crt";
    let private_key = "ssl-keys/rustasync.key";

    // load ssl keys
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert_file = &mut BufReader::new(File::open(certificate).unwrap());
    let key_file = &mut BufReader::new(File::open(private_key).unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();

    println!("running on: https://localhost:8088");

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_rustls("127.0.0.1:8088", config)?
        .run()
        .await
}
