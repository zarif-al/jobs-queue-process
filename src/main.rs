extern crate dotenv;

mod db_connect;
mod root_route;

use axum::{routing::post, Router};
use dotenv::dotenv;
use redis_work_queue::{KeyPrefix, WorkQueue};
use serde::Serialize;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

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
    let (tx, rx) = mpsc::channel::<String>(32);

    // build our application with a single route
    let app = Router::new().route(
        "/",
        post({
            let tx_clone = tx.clone();
            move || root_route::handle(tx_clone)
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
