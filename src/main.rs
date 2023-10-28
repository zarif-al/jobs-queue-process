mod client;
mod db_connect;
mod env_config;
mod job_process;
mod payload;
mod root_route;

use axum::{extract::Json, routing::post, Router};
use redis_work_queue::{KeyPrefix, WorkQueue};
use payload::RequestPayload;
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
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

    // create work queue
    let work_queue = Arc::new(WorkQueue::new(KeyPrefix::from(env_config.redis_work_queue)));

    // transmitters and receivers for job queue thread
    let (tx, rx) = mpsc::channel::<RequestPayload>(32);

    // build our application with a single route
    let app = Router::new().route(
        "/",
        post(move |Json(payload): Json<RequestPayload>| root_route::handle(tx, payload)),
    );

    // thread to listen and add jobs to queue
    tokio::spawn(root_route::queue_thread(
        String::from("Route: '/' Thread"),
        rx,
        Arc::clone(&work_queue),
    ));

    // thread to process jobs
    tokio::spawn(root_route::process_thread(
        String::from("Process Thread 1"),
        Arc::clone(&work_queue),
    ));

    let addr = SocketAddr::from(([127, 0, 0, 1], env_config.port));
    info!("App listening on {}", addr);
    // serve it with hyper on designated port
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("App failed to startup!");
}
