use std::{sync::Arc, time::Duration};

use redis_work_queue::WorkQueue;
use tracing::{error, info, warn};

use crate::db::{mongo_message, mongo_message::DBMessage, redis_conn};

pub async fn process_jobs(name: String, work_queue: Arc<WorkQueue>) {
    let redis_conn = redis_conn().await;

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
                            match job.data_json::<DBMessage>() {
                                Ok(data) => {
                                    // call db_insert with response data
                                    match mongo_message::insert(data.message, data.email).await {
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
                                Err(_) => {
                                    // TODO: Re-think error handling
                                    panic!("Could not process!")
                                }
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
