use warp::{Filter, http};
use warp::http::header::{HeaderMap, HeaderValue};

#[macro_use]
extern crate log;

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


// // @see https://github.com/seanmonstar/warp/issues/448#issuecomment-587174177
// fn proxy(
//     client: hyper::Client,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     path_and_query()
//         .and(warp::method())
//         .and(header::headers_cloned())
//         .and(warp::body::stream())
//         .and(with_client(client))
//         .and_then(handlers::proxy_request)
// }
mod filters {
    //use hyper::Client;
    use super::handlers;
    use warp::Filter;

    /// @see https://github.com/seanmonstar/warp/issues/448#issuecomment-587174177
    pub fn proxy(
        client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        
        let http_client = warp::any().map(move || client.clone());

        warp::any()
            .and(warp::path::full())
            .and(warp::method())
            .and(warp::header::headers_cloned())
            .and(warp::body::stream())
            .and(http_client)
            .and_then(handlers::proxy_request)
    }
}


mod handlers {
    use warp::http::header::HeaderMap;
    use futures::stream::Stream;
    use futures::TryStreamExt;

    #[derive(Debug)]
    struct HyperClientError;

    impl warp::reject::Reject for HyperClientError {}

    pub async fn proxy_request(
        path: warp::path::FullPath,
        method: http::Method,
        headers: HeaderMap,
        body: impl Stream<Item = Result<impl hyper::body::Buf, warp::Error>> + Send + Sync + 'static,
        // body: impl Stream<Item = Result<impl hyper::body::Buf, warp::Error>> + Send + Sync + 'static,
        client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let body = body.map_ok(|mut buf| {
            buf.to_bytes()
        });
        let url = path.as_str();

        debug!("url: {}", &url);

        let mut request = hyper::Request::builder()
            .uri(url)
            .method(method)
            .body(hyper::Body::wrap_stream(body))
            .unwrap();

        *request.headers_mut() = headers;
        let response = client.request(request).await;

        match response {
            Ok(response) => Ok(response),
            Err(_) => {
                Err(warp::reject::custom(HyperClientError))
            },
        }
        // Ok(response.unwrap())
    }
}



#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // Match any request and return hello world!
    // let routes = warp::any().map(|| "Hello, World!");
    // let routes = warp::any().and_then(proxy_request);

    let https = hyper_rustls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
//    let client = hyper::Client::new();
    let routes = filters::proxy(client.clone());

    warp::serve(routes)
        .tls()
        .cert_path("ssl-keys/rustasync.crt")
        .key_path("ssl-keys/rustasync.key")
        .run(([127, 0, 0, 1], 3030))
        .await;
}
