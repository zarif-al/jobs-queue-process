use std::sync::Arc;

use redis_work_queue::{Item, WorkQueue};
use tokio::sync::mpsc::Receiver;
use tracing::info;

use crate::{db_connect, shopify::RequestPayload};

/*
 This thread will receive jobs from the `/` route handler and
 add it to queue.
*/
pub async fn queue_thread(
    name: String,
    mut rx: Receiver<RequestPayload>,
    work_queue: Arc<WorkQueue>,
) {
    // Get db connection
    match db_connect::redis_conn().await {
        Some(mut conn) => {
            info!("{} => Ready to receive jobs!", name);

            // await to receive jobs from the sender channel in `/`
            // route handler.
            // add those jobs to queue.
            // run in an infinite loop.
            loop {
                for received in rx.recv().await.iter() {
                    let job = Item::from_json_data(received).unwrap();

                    work_queue
                        .add_item(&mut conn, &job)
                        .await
                        .expect("{name} => Failed to add job to queue.");

                    info!("{name} => Added job to queue. Job ID: {}", job.id,);
                }
            }
        }
        None => {}
    }
}
