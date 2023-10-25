pub mod handle;
pub mod processing_thread;
pub mod queue_thread;

// TODO: Revisit this
// Implementation One: Re-add to queue.
// This causes an infinite loop of trying and failing the job. We need a way
// to track how many times a job has been tried.
//
// Implementation Two: Push to a mongo db instance.
// async fn handle_failure(name: String, job: Item, work_queue: Arc<WorkQueue>) {
//     // match db_connect::redis_conn().await {
//     //     Some(mut conn) => {
//     //         warn!(
//     //             "{} => Marking job: {} as complete! This is a temporary workaround.",
//     //             name, job.id
//     //         );

//     //         work_queue
//     //             .complete(&mut conn, &job)
//     //             .await
//     //             .expect("Failed to mark a job as incomplete!");

//     //         warn!(
//     //             "{} => Adding job: {} to queue! This is a temporary workaround.",
//     //             name, job.id
//     //         );

//     //         let item = Item::new(job.data);

//     //         work_queue
//     //             .add_item(&mut conn, &item)
//     //             .await
//     //             .expect("Failed to re-add job to queue");
//     //     }
//     //     None => {
//     //         error!(
//     //             "{} => Failed to handle job failure. Job Id: {}",
//     //             name, job.id
//     //         );
//     //     }
//     // }
//     todo!("Code up implementation two");
// }
