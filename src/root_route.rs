use std::time::Duration;

use axum::{http::StatusCode, Json};
use redis_work_queue::{Item, KeyPrefix, WorkQueue};
use serde::Serialize;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle(tx: Sender<String>) -> (StatusCode, Json<Response>) {
    tx.send(String::from("job"))
        .await
        .expect("Expected: Send job down the channel.");

    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}

pub async fn queue_thread(name: String, mut rx: Receiver<String>) {
    let host = "localhost";
    let db = &mut redis::Client::open(format!("redis://{host}/"))
        .expect("Expected: Rust db")
        .get_async_connection()
        .await
        .expect("Expected: Async connection to Rust db");

    let work_queue = WorkQueue::new(KeyPrefix::from("sanity_custom_sync_rust"));

    loop {
        for received in rx.recv().await.iter() {
            let job = Item::from_string_data(String::from(received));

            work_queue
                .add_item(db, &job)
                .await
                .expect("Expected: Successfull addition of job to queue");

            println!("{name} => Added job to queue. Job ID: {}", job.id);
        }
    }
}

pub async fn processing_thread(name: String) {
    const PROCESSING_TIME: Duration = Duration::from_secs(30 * 10);

    // connect to redis
    let host = "localhost";
    let db = &mut redis::Client::open(format!("redis://{host}/"))
        .expect("Expected: Rust db")
        .get_async_connection()
        .await
        .expect("Expected: Async connection to Rust db");

    // get work queue
    let work_queue = WorkQueue::new(KeyPrefix::from("sanity_custom_sync_rust"));

    loop {
        let job: Option<Item> = work_queue
            .lease(db, None, Duration::from_secs(60))
            .await
            .expect("Expected: Lease a job");

        match job {
            Some(job) => {
                println!("{name} => Processing Job: {}", job.id);
                sleep(PROCESSING_TIME).await;
                work_queue
                    .complete(db, &job)
                    .await
                    .expect("Expected: Mark job as complete from: processing thread 1");
                println!("{name} => Completed Processing Job: {}", job.id);
            }
            None => {}
        }
    }
}
