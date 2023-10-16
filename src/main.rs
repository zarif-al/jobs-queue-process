mod root_route;

use axum::{routing::post, Router};
use serde::Serialize;
use tokio::sync::mpsc;
#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
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
    tokio::spawn(root_route::queue_thread(String::from("Route Thread 1"), rx));

    // threads to process jobs from queue
    tokio::spawn(root_route::processing_thread(String::from(
        "Processing Thread 1",
    )));
    tokio::spawn(root_route::processing_thread(String::from(
        "Processing Thread 2",
    )));
    tokio::spawn(root_route::processing_thread(String::from(
        "Processing Thread 3",
    )));

    // run it with hyper on localhost:4000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
