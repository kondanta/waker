use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};

use serde::Serialize;

mod wol;
mod util;

// Struct that holds the string value of MAC address

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/wol", get(wol));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Root endpoint
async fn root() -> &'static str {
    "Root page!"
}

// WoL endpoint
async fn wol() -> (StatusCode, Json<Response<'static>>) {
    wol::create_wol_message().ok();
    (StatusCode::OK, Json(Response { message: "Magic packet sent!" }))
}