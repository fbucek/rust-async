use futures_util::future::TryFutureExt;
use warp::Filter;

#[macro_use]
extern crate log;

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
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let http_client = reqwest::Client::new();
    let http_client = warp::any().map(move || http_client.clone());

    let call_route = warp::path::path("call")
        .and(http_client.clone())
        .and_then(call_wrapper);

    warp::serve(call_route)        
        .tls()
        .cert_path("ssl-keys/rustasync.crt")
        .key_path("ssl-keys/rustasync.key")
        .run(([127, 0, 0, 1], 9000)).await;
}

/// Wrap the actual function so we only have to call reject::custom once
async fn call_wrapper(http: reqwest::Client) -> Result<impl warp::Reply, warp::Rejection> {
    call_site(http).map_err(warp::reject::custom).await
}

async fn call_site(http: reqwest::Client) -> Result<String, MyError> {
    let url: String = "https://api.github.com/users/octocat/orgs".into();
    let resp = http
        .get(&url)
        .send()
        .await?;
    
    debug!("response: {:?}", &resp.status());

    let body = &resp.text().await?;
    
    Ok(format!("Got a response with length: {}", body.len()))
}
