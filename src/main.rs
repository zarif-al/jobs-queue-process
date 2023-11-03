mod client;
mod db_connect;
mod env_config;
mod messages_route;
mod post_job_route;
mod processor;
mod req_res_structs;

use axum::{
    extract::{Json, Query},
    routing::{get, post},
    Router,
};
use redis_work_queue::{KeyPrefix, WorkQueue};
use req_res_structs::{MessagesRequestPayload, PostJobRequestPayload};
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

    // get env config
    let env_config = env_config::get_env_config();

    // create work queue
    let work_queue = Arc::new(WorkQueue::new(KeyPrefix::from(env_config.redis_work_queue)));

    // transmitters and receivers to pass job to queue thread
    let (tx, rx) = mpsc::channel::<PostJobRequestPayload>(32);

    // build our application
    let app = Router::new()
        .route(
            "/post-job",
            post(move |Json(payload): Json<PostJobRequestPayload>| {
                post_job_route::handle(tx, payload)
            }),
        )
        .route(
            "/messages",
            get(move |Query(payload): Query<MessagesRequestPayload>| {
                messages_route::handle(payload)
            }),
        );

    // thread to listen and add jobs to queue
    tokio::spawn(post_job_route::queue_thread(
        String::from("Route: '/' Thread"),
        rx,
        Arc::clone(&work_queue),
    ));

    // thread to process jobs
    tokio::spawn(post_job_route::process_thread(
        String::from("Process Thread 1"),
        Arc::clone(&work_queue),
    ));

    // setup server address
    let addr = SocketAddr::from(([127, 0, 0, 1], env_config.port));
    info!("App listening on {}", addr);

    // serve it with hyper on designated port
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("App failed to startup!");
}
