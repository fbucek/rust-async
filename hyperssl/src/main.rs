use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

// OpenSSL
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

type Res<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

//f/ Builds an SSL implementation for Simple HTTPS rom some hard-coded file names
/// @see https://docs.rs/openssl/0.10.26/openssl/ssl/index.html
#[allow(dead_code)]
fn ssl() -> Res<SslAcceptorBuilder> {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    acceptor.set_private_key_file("keys/hyper-localhost.key", SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file("keys/hyper-localhost.crt")?;
    acceptor.check_private_key()?;

    Ok(acceptor)
}

#[tokio::main]
pub async fn main() -> Res<()> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
