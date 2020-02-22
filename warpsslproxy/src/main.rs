use warp::{Filter, http};
use warp::http::header::{HeaderMap, HeaderValue};
use futures::stream::Stream;

use futures::TryStreamExt;
//use futures_util::stream::Stream;
// use hyper::HttpClient;


// @see https://github.com/seanmonstar/warp/issues/319#issue-525659230


// fn extract_request(
// ) -> impl Filter<Extract = (http::Request<warp::body::BodyStream>,), Error = warp::Rejection> + Copy
// {
//     warp::method()
//         .and(warp::path::full())
//         .and(warp::headers::headers_cloned())
//         .and(warp::body::stream())
//         .map(
//             |method: http::Method,
//             path: warp::path::FullPath,
//             headers: http::HeaderMap,
//             body: warp::body::BodyStream| {
//                 let mut req = http::Request::builder()
//                     .method(method)
//                     .uri(path.as_str())
//                     .body(body)
//                     .expect("request builder");
//                 *req.headers_mut() = headers;
//                 req * req.method_mut() = method;
//             },
//         )
// }


// @see https://github.com/seanmonstar/warp/issues/448#issuecomment-587174177
// fn proxy(
//     client: HttpClient,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     path_and_query()
//         .and(warp::method())
//         .and(header::headers_cloned())
//         .and(warp::body::stream())
//         .and(with_client(client))
//         .and_then(handlers::proxy_request)
// }

async fn proxy_request(
    url: url::Url,
    method: http::Method,
    headers: HeaderMap,
    body: impl Stream<Item = Result<impl hyper::body::Buf, warp::Error>> + Send + Sync + 'static,
    // body: impl Stream<Item = Result<impl hyper::body::Buf, warp::Error>> + Send + Sync + 'static,
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let body = body.map_ok(|mut buf| {
        buf.to_bytes()
    });
    let mut request = hyper::Request::builder()
        .uri(url.as_str())
        .method(method)
        .body(hyper::Body::wrap_stream(body))
        .unwrap();

    *request.headers_mut() = headers;
    let response = client.request(request).await;

    Ok(response.unwrap())
}

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");
    //let routes = warp::any().map(proxy_request);

    warp::serve(routes)
        .tls()
        .cert_path("ssl-keys/rustasync.crt")
        .key_path("ssl-keys/rustasync.key")
        .run(([127, 0, 0, 1], 3030))
        .await;
}
