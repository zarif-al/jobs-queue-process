extern crate dotenv;

mod connection_check;
mod root_route;

use axum::{routing::post, Router};
use dotenv::dotenv;
use redis_work_queue::{KeyPrefix, WorkQueue};
use serde::Serialize;
use std::env;
use tokio::sync::mpsc;
#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Load env form .env
    dotenv().ok();

    // Check env
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env");

    // Check connections
    let redis_connected = connection_check::redis_conn_check(&redis_url).await;

    let redis_conn;
    // let redis_ref;
    match redis_connected {
        Some(conn) => {
            redis_conn = conn;
            // redis_ref = Arc::new(Mutex::new(conn));
            println!("DB Connected!");
        }
        None => panic!("Could not establish connection to db."),
    }

    let work_queue = WorkQueue::new(KeyPrefix::from("sanity_custom_sync_rust"));

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
        redis_conn,
        work_queue,
    ));

    // threads to process jobs from queue
    // tokio::spawn(root_route::processing_thread(
    //     "Processing Thread 1".to_string(),
    //     redis_conn,
    // ));
    // tokio::spawn(root_route::processing_thread(
    //     "Processing Thread 2".to_string(),
    //     redis_conn,
    // ));
    // tokio::spawn(root_route::processing_thread(
    //     "Processing Thread 3".to_string(),
    //     redis_conn,
    // ));

    // run it with hyper on localhost:4000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
