use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes)
        .tls()
        .cert_path("keys/warp-localhost.crt")
        .key_path("keys/warp-localhost.key")
        .run(([127, 0, 0, 1], 3030))
        .await;
}