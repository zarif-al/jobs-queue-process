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
 Success: Returns an OK response
 Failure: Returns an Error response

 TODO: This handler does not get called if the JSON parse fails.
*/
pub async fn handle(
    tx: Sender<PostJobRequestPayload>,
    payload: PostJobRequestPayload,
) -> (StatusCode, Json<GeneralResponse>) {
    // Send to thread to add to queue.
    match tx.send(payload).await {
        Ok(_) => {
            // Return an OK response
            (
                StatusCode::OK,
                Json(GeneralResponse {
                    message: Some(String::from("OK")),
                    error: None,
                }),
            )
        }
        Err(err) => {
            error!("Failed to send job down the channel. Error: {}", err);
            // Return an ERROR response
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GeneralResponse {
                    message: None,
                    error: Some(String::from("Failed to process request.")),
                }),
            )
        }
    }
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
                    // TODO: Should we re-think how to handle failure here
                    match Item::from_json_data(received) {
                        Ok(job) => {
                            // add job to queue
                            match work_queue.add_item(&mut conn, &job).await {
                                Ok(_) => {
                                    info!("{} => Added job to queue. Job ID: {}", name, job.id);
                                }
                                Err(err) => {
                                    error!(
                                        "{} => Failed to add job to queue. Error: {}",
                                        name, err
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            error!("{name} => Failed to create job. Error: {}", err)
                        }
                    };
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
                match work_queue
                    .lease(
                        &mut conn,
                        Some(Duration::from_secs(5)),
                        Duration::from_secs(60),
                    )
                    .await
                {
                    Ok(job) => match job {
                        Some(job) => {
                            info!("{} => Processing Job: {}", name, job.id,);
                            match job.data_json::<PostJobRequestPayload>() {
                                Ok(data) => {
                                    // call db_insert with response data
                                    match processor::db_insert(data.message, data.email).await {
                                        Some(()) => {
                                            // Mark job as completed if db_insert returns Some()
                                            match work_queue.complete(&mut conn, &job).await {
                                                Ok(_) => {
                                                    info!(
                                                        "{} => Completed processing job: {}",
                                                        name, job.id
                                                    );
                                                }
                                                Err(err) => {
                                                    error!(
                                                    "{} => Failed to mark a job as incomplete. Error: {}",
                                                    name,err
                                                )
                                                }
                                            }
                                        }
                                        None => {
                                            // Re-queue job if db_insert returns None
                                            match work_queue.add_item(&mut conn, &job).await {
                                                Ok(_) => {
                                                    warn!("{} => Re-queued job: {}", name, job.id);
                                                }
                                                Err(err) => {
                                                    error!("{} => Failed to re-queue job after failure. Error: {}", name,err);
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(_) => panic!("Could not process!"),
                            };
                        }
                        None => continue,
                    },
                    Err(err) => {
                        error!("Failed to lease job. Error: {}", err);
                    }
                }
            }
        },
        None => error!("{} => Failed to connect to redis db", name),
    }
}
