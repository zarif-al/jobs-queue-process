use axum::{http::StatusCode, Json};
use std::{sync::Arc, time::Duration};

use redis_work_queue::{Item, WorkQueue};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info, warn};

use crate::{
    db_connect, processor, req_res_structs::GeneralResponse, req_res_structs::PostJobRequestPayload,
};

/*
 This handler will accept the body of a post request and pass it along
 to a thread.
 Then it will respond with an OK status.
 Caution: This handler does not get called if the JSON parse fails.
*/
pub async fn handle(
    tx: Sender<PostJobRequestPayload>,
    payload: PostJobRequestPayload,
) -> (StatusCode, Json<GeneralResponse>) {
    // job_process("Testing".to_string(), "zarif_al96@outlook.com".to_string()).await;
    // Send to thread to add to queue.
    tx.send(payload)
        .await
        .expect("Failed to send job down the channel");

    // Return OK response
    (
        StatusCode::OK,
        Json(GeneralResponse {
            message: Some(String::from("OK")),
            error: None,
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
    mut rx: Receiver<PostJobRequestPayload>,
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

pub async fn process_thread(name: String, work_queue: Arc<WorkQueue>) {
    let redis_conn = db_connect::redis_conn().await;

    match redis_conn {
        Some(mut conn) => loop {
            info!("{} => Ready to process jobs!", name);
            loop {
                let job: Option<Item> = work_queue
                    .lease(
                        &mut conn,
                        Some(Duration::from_secs(5)),
                        Duration::from_secs(60),
                    )
                    .await
                    .expect("Failed to lease a job!");

                match job {
                    Some(job) => {
                        info!("{} => Processing Job: {}", name, job.id,);

                        let job_data = match job.data_json::<PostJobRequestPayload>() {
                            Ok(response) => response,
                            Err(_) => panic!("Could not process!"),
                        };

                        // call db_insert with job data
                        match processor::db_insert(job_data.message, job_data.email).await {
                            Some(()) => {
                                // Mark job as completed if db_insert returns Some()
                                work_queue
                                    .complete(&mut conn, &job)
                                    .await
                                    .expect("Failed to mark a job as incomplete.");
                                info!("{} => Completed processing job: {}", name, job.id);
                            }
                            None => {
                                // Re-queue job if db_insert returns None
                                work_queue
                                    .add_item(&mut conn, &job)
                                    .await
                                    .expect("Failed to re-queue job after failure.");

                                warn!("{} => Re-queing job: {}", name, job.id);
                            }
                        }
                    }
                    None => continue,
                }
            }
        },
        None => error!("{} => Failed to connect to redis db", name),
    }
}
