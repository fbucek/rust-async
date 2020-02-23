use futures::stream::Stream;
use futures::TryStreamExt; // body.map_ok(|mut buf|
use warp::http::header::HeaderMap;
use warp::Filter;

#[macro_use]
extern crate log;

#[derive(Debug)]
struct HyperClientError;

impl warp::reject::Reject for HyperClientError {}

pub async fn handler_proxy(
    path: warp::path::FullPath,
    method: http::Method,
    headers: HeaderMap,
    body: impl Stream<Item = Result<impl hyper::body::Buf, warp::Error>> + Send + Sync + 'static,
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let body = body.map_ok(|mut buf| buf.to_bytes());
    let url = format!("https://api.github.com{}", path.as_str());

    debug!("url: {}", &url);
    debug!("method: {:?}", &method);

    let mut request = hyper::Request::builder()
        .uri(url)
        .method(method)
        .body(hyper::Body::wrap_stream(body))
        .unwrap();

    *request.headers_mut() = headers;
    trace!("resp: {:?}", request);

    // Get data from server
    let response = client.request(request).await;

    debug!("client finished");

    match response {
        // Return response data
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::custom(HyperClientError)),
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warpsslproxyhyper1=debug");
    env_logger::init();

    let https = hyper_rustls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let http_client = warp::any().map(move || client.clone());

    let routes = warp::any()
        .and(warp::path::full()) // path: /users/octocat/orgs
        .and(warp::method()) // GET, POST
        .and(warp::header::headers_cloned()) // headers
        .and(warp::body::stream()) // body
        .and(http_client) // hyper::Client
        .and_then(handler_proxy);

    warp::serve(routes)
        .tls()
        .cert_path("ssl-keys/rustasync.crt")
        .key_path("ssl-keys/rustasync.key")
        .run(([127, 0, 0, 1], 3030))
        .await;
}
