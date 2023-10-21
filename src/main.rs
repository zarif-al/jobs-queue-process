mod client;
mod db_connect;
mod env_config;
mod root_route;
mod sanity;
mod shopify_payload;

use axum::{extract::Json, routing::post, Router};
use env_config::get_env_config;
use redis_work_queue::{KeyPrefix, WorkQueue};
use shopify_payload::RequestPayload;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // get env config
    let env_config = get_env_config();

    // create work queue
    let work_queue = Arc::new(WorkQueue::new(KeyPrefix::from(env_config.redis_work_queue)));

    // transmitters and receivers
    let (tx, rx) = mpsc::channel::<RequestPayload>(32);

    // We should prevent the app from proceeding until we have a connection. Otherwise
    // Multiple threads will try to connect to the db at the same time.
    if db_connect::redis_conn().await.is_none() {
        error!("Redis connection failed");
        panic!("Failed to connect to db");
    }

    info!("Redis Connection successfull");

    // build our application with a single route
    let app = Router::new().route(
        "/",
        post({
            let tx_clone = tx.clone();
            move |Json(payload): Json<RequestPayload>| root_route::handle(tx_clone, payload)
        }),
    );

    // thread to listen and add jobs to queue
    tokio::spawn(root_route::queue_thread(
        String::from("Route Thread 1"),
        rx,
        Arc::clone(&work_queue),
    ));

    // threads to process jobs from queue
    tokio::spawn(root_route::processing_thread(
        "Processing Thread 1".to_string(),
        Arc::clone(&work_queue),
    ));
    tokio::spawn(root_route::processing_thread(
        "Processing Thread 2".to_string(),
        Arc::clone(&work_queue),
    ));
    tokio::spawn(root_route::processing_thread(
        "Processing Thread 3".to_string(),
        Arc::clone(&work_queue),
    ));

    // serve it with hyper on localhost
    axum::Server::bind(&format!("0.0.0.0:{}", env_config.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
