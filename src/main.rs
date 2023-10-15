mod root_handle;

use std::time::Duration;

use axum::{routing::post, Router};
use redis_work_queue::{Item, KeyPrefix, WorkQueue};
use serde::Serialize;
use tokio::{sync::mpsc, time::sleep};
#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    const PROCESSING_TIME: Duration = Duration::from_secs(30 * 10);
    // transmitters and receivers
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // build our application with a single route
    let app = Router::new().route(
        "/",
        post({
            let tx_clone = tx.clone();
            move || root_handle::handle(tx_clone)
        }),
    );

    // thread to listen and add jobs to queue
    tokio::spawn(async move {
        // connect to redis
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

                println!("Route Thread => Added job to queue. Job ID: {}", job.id);
            }
        }
    });

    // thread to process jobs from queue
    tokio::spawn(async {
        // connect to redis
        let host = "localhost";
        let db = &mut redis::Client::open(format!("redis://{host}/"))
            .expect("Expected: Rust db")
            .get_async_connection()
            .await
            .expect("Expected: Async connection to Rust db");

        let work_queue = WorkQueue::new(KeyPrefix::from("sanity_custom_sync_rust"));

        loop {
            let job: Option<Item> = work_queue
                .lease(db, None, Duration::from_secs(60))
                .await
                .expect("Expected: Lease a job");

            match job {
                Some(job) => {
                    println!("Processing Thread 1 => Processing Job: {}", job.id);
                    sleep(PROCESSING_TIME).await;
                    work_queue
                        .complete(db, &job)
                        .await
                        .expect("Expected: Mark job as complete from: processing thread 1");
                    println!(
                        "Processing Thread 1 => Completed Processing Job: {}",
                        job.id
                    );
                }
                None => {}
            }
        }
    });

    // thread to process jobs from queue
    tokio::spawn(async {
        // connect to redis
        let host = "localhost";
        let db = &mut redis::Client::open(format!("redis://{host}/"))
            .expect("Expected: Rust db")
            .get_async_connection()
            .await
            .expect("Expected: Async connection to Rust db");

        let work_queue = WorkQueue::new(KeyPrefix::from("sanity_custom_sync_rust"));

        loop {
            let job: Option<Item> = work_queue
                .lease(db, None, Duration::from_secs(60))
                .await
                .expect("Expected: Lease a job");

            match job {
                Some(job) => {
                    println!("Processing Thread 2 => Processing Job: {}", job.id);
                    sleep(PROCESSING_TIME).await;
                    work_queue
                        .complete(db, &job)
                        .await
                        .expect("Expected: Mark job as complete from processing thread 2.");
                    println!(
                        "Processing Thread 2 => Completed Processing Job: {}",
                        job.id
                    );
                }
                None => {}
            }
        }
    });
    // run it with hyper on localhost:4000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
