use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Pessoa {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_pessoas));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_pessoas() -> Json<Vec<Pessoa>> {
    let messages = vec![
        Pessoa {
            name: "tales".to_string(),
            age: 20,
        },
        Pessoa {
            name: "tales".to_string(),
            age: 20,
        },
    ];

    Json(messages)
}
