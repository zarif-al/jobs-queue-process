mod client;
mod db_connect;
mod env_config;
mod root_handle;

use axum::{routing::post, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", post(root_handle::handle));

    // run it with hyper on localhost:4000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
