use futures::TryStreamExt;
use warp::http::header::HeaderMap;
use warp::Filter;

#[macro_use]
extern crate log;
use std::io::Write;

#[derive(Debug)]
struct HyperClientError;

impl warp::reject::Reject for HyperClientError {}

#[derive(Debug)]
enum MyError {
    Http(reqwest::Error),
}

impl warp::reject::Reject for MyError {}
impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::Http(err)
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warpsslproxyreqwest=debug");
    // @see <https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html>
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {} [{}]",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.args(),
                record.module_path().unwrap(),
            )
        })
        .init();
    // let mut builder = env_logger::Builder::from_default_env()
    //     .format_timestamp_millis()
    //     .init();
    // env_logger::init()

    let http_client = reqwest::Client::new();
    let http_client = warp::any().map(move || http_client.clone());

    let routes = warp::any()
        .and(warp::path::full()) // path: /users/octocat/orgs
        .and(warp::method()) // GET, POST
        .and(warp::header::headers_cloned()) // headers
        .and(warp::body::stream()) // body
        .and(http_client.clone())
        .and_then(handler_proxy);

    info!("Starting server: https://localhost:3030");

    warp::serve(routes)
        .tls()
        .cert_path("ssl-keys/rustasync.crt")
        .key_path("ssl-keys/rustasync.key")
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// /// Wrap the actual function so we only have to call reject::custom once
// async fn call_wrapper(http: reqwest::Client) -> Result<impl warp::Reply, warp::Rejection> {
//     handler_proxy(http).map_err(warp::reject::custom).await
// }

async fn handler_proxy(
    path: warp::path::FullPath,
    method: warp::http::Method,
    headers: HeaderMap,
    body: impl futures::stream::Stream<Item = Result<impl hyper::body::Buf, warp::Error>>
        + Send
        + Sync
        + 'static,
    client: reqwest::Client,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Map stream from hyper::body::Buf to bytes
    let body_bytes = body.map_ok(|mut buf| buf.to_bytes());
    let url = format!("https://api.github.com{}", path.as_str());

    debug!("url: {}", &url);
    debug!("method: {:?}", &method);

    // let body: reqwest::Body = body_bytes.into();
    let body = reqwest::Body::wrap_stream(body_bytes);

    let resp = client
        .request(method, &url)
        .headers(headers)
        .body(body)
        .send()
        .await;

    match resp {
        Ok(resp) => {
            // Prepare response with header
            let headers = resp.headers().clone();
            let body: String = resp.text().await.unwrap();
            let mut response = warp::http::Response::builder().body(body).unwrap();

            *response.headers_mut() = headers;

            Ok(response)
        }
        Err(_) => Err(warp::reject::custom(HyperClientError)),
    }
}
