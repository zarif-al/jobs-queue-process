extern crate dotenv;

mod db_connect;
mod root_route;
mod shopify_payload;

use axum::{extract::Json, routing::post, Router};
use dotenv::dotenv;
use redis_work_queue::{KeyPrefix, WorkQueue};
use shopify_payload::RequestPayload;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // load env form .env
    dotenv().ok();

    // get redis work queue name
    let redis_work_queue_name =
        env::var("REDIS_WORK_QUEUE").expect("REDIS_WORK_QUEUE is not set in .env");

    // create work queue
    let work_queue = Arc::new(WorkQueue::new(KeyPrefix::from(redis_work_queue_name)));

    // transmitters and receivers
    let (tx, rx) = mpsc::channel::<RequestPayload>(32);

    // We should prevent the app from proceeding until we have a connection. Otherwise
    // Multiple threads will try to connect to the db at the same time.
    if db_connect::redis_conn().await.is_none() {
        panic!("Failed to connect to db");
    }

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

    // get app port
    let port = env::var("PORT").expect("PORT not set in .evn");

    // serve it with hyper on localhost
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
