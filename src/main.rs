mod client;
mod db_connect;
mod env_config;
mod root_route;
mod sanity;
mod shopify_payload;

use axum::{routing::post, Json, Router};
use serde::Serialize;
use shopify_payload::RequestPayload;
use tracing::info;
use tracing_subscriber;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // check env config
    let env_config = env_config::get_env_config();

    // build our application with a single route
    let app = Router::new().route(
        "/",
        post(move |Json(payload): Json<RequestPayload>| root_route::handle(payload)),
    );

    // serve it with hyper on designated port
    axum::Server::bind(&format!("0.0.0.0:{}", env_config.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    info!("App is ready to serve!");
}
