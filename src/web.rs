use axum::routing::get;
use std::net::SocketAddr;

#[tokio::main]
pub async fn run() {
    let app = axum::Router::new().route(
        "/",
        get(|| async {
            {
                "Hello world!"
            }
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
