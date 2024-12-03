use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async {
            let lista = ["Hello world", "Hello wolrd2"];
            lista[1]
        }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}