use std::sync::Arc;

use redis_work_queue::{Item, WorkQueue};
use tokio::sync::mpsc::Receiver;
use tracing::{error, info};

use crate::db::{mongo_entities::DBMessage, redis_conn};

/*
 This thread will receive jobs from the `/` route handler and
 add it to queue.

 This thread will run in an infinite loop only if a connection to
 the db is established.

 Otherwise it will panic!().
*/
pub async fn queue_jobs(name: String, mut rx: Receiver<DBMessage>, work_queue: Arc<WorkQueue>) {
    match redis_conn().await {
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
            // TODO: Re-think error handling
            panic!("{} => Failed to connect to db.", name);
        }
    }
}
