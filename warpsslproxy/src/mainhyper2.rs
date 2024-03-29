#[macro_use]
extern crate log;

mod filters {
    //use hyper::Client;
    use super::handlers;
    use warp::Filter;

    /// @see <https://github.com/seanmonstar/warp/issues/448#issuecomment-587174177>
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
    use futures::stream::Stream;
    use futures::TryStreamExt;
    use warp::http::header::HeaderMap;

    #[derive(Debug)]
    struct HyperClientError;

    impl warp::reject::Reject for HyperClientError {}

    pub async fn proxy_request(
        path: warp::path::FullPath,
        method: warp::http::Method,
        headers: HeaderMap,
        body: impl Stream<Item = Result<impl hyper::body::Buf, warp::Error>> + Send + Sync + 'static,
        // body: impl Stream<Item = Result<impl hyper::body::Buf, warp::Error>> + Send + Sync + 'static,
        client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let body = body.map_ok(|mut buf| buf.to_bytes());
        let url = format!("https://api.github.com{}", path.as_str());

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
            Err(_) => Err(warp::reject::custom(HyperClientError)),
        }
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warpsslproxyhyper2=debug");
    env_logger::init();

    let https = hyper_rustls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    //let http_client = warp::any().map(move || client.clone());

    let routes = filters::proxy(client.clone());

    warp::serve(routes)
        .tls()
        .cert_path("ssl-keys/rustasync.crt")
        .key_path("ssl-keys/rustasync.key")
        .run(([127, 0, 0, 1], 3030))
        .await;
}
