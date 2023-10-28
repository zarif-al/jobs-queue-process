use axum::{http::StatusCode, Json};
use mongodb::bson::doc;
use serde::Serialize;
use std::sync::Arc;

use redis_work_queue::{Item, WorkQueue};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;

use crate::{db_connect, request_payload::RequestPayload};

#[derive(Serialize)]
pub struct Response {
    message: String,
}

/*
 This handler will accept the body of a post request and pass it along
 to a thread.
 Then it will respond with an OK status.
 Caution: This handler does not get called if the JSON parse fails.
*/
pub async fn handle(
    tx: Sender<RequestPayload>,
    payload: RequestPayload,
) -> (StatusCode, Json<Response>) {
    // job_process("Testing".to_string(), "zarif_al96@outlook.com".to_string()).await;
    // Send to thread to add to queue.
    tx.send(payload)
        .await
        .expect("Failed to send job down the channel");

    // Return OK response
    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}

/*
 This thread will receive jobs from the `/` route handler and
 add it to queue.

 This thread will run in an infinite loop only if a connection to
 the db is established.

 Otherwise it will panic!().
*/
pub async fn queue_thread(
    name: String,
    mut rx: Receiver<RequestPayload>,
    work_queue: Arc<WorkQueue>,
) {
    match db_connect::redis_conn().await {
        Some(mut conn) => {
            info!("{} => Ready to receive jobs!", name);

            loop {
                for received in rx.recv().await.iter() {
                    let job = Item::from_json_data(received).unwrap();

                    // add job to queue
                    work_queue
                        .add_item(&mut conn, &job)
                        .await
                        .expect("{name} => Failed to add job to queue.");

                    info!("{name} => Added job to queue. Job ID: {}", job.id,);
                }
            }
        }
        None => {
            panic!("{} => Failed to connect to db.", name);
        }
    }
}
