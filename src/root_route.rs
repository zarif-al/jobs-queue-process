use crate::db_connect;
use crate::shopify_payload::RequestPayload;

use axum::{extract::Json, http::StatusCode};
use redis_work_queue::{Item, WorkQueue};
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle(
    tx: Sender<RequestPayload>,
    payload: RequestPayload,
) -> (StatusCode, Json<Response>) {
    tx.send(payload)
        .await
        .expect("Failed to send job down the channel");

    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}

pub async fn queue_thread(
    name: String,
    mut rx: Receiver<RequestPayload>,
    work_queue: Arc<WorkQueue>,
) {
    match db_connect::redis_conn().await {
        Some(mut conn) => {
            println!("{name} => Ready to receive jobs!");

            loop {
                for received in rx.recv().await.iter() {
                    let job = Item::from_json_data(received).unwrap();

                    work_queue
                        .add_item(&mut conn, &job)
                        .await
                        .expect("{name} => Failed to add job to queue.");

                    println!("{name} => Added job to queue. Job ID: {}", job.id,);
                }
            }
        }
        None => {}
    }
}

pub async fn processing_thread(name: String, work_queue: Arc<WorkQueue>) {
    const PROCESSING_TIME: Duration = Duration::from_secs(10);

    match db_connect::redis_conn().await {
        Some(mut conn) => {
            println!("{name} => Ready to process jobs!");

            loop {
                let job: Option<Item> = work_queue
                    .lease(&mut conn, None, Duration::from_secs(60))
                    .await
                    .expect("Failed to lease a job!");

                match job {
                    Some(job) => {
                        println!("{name} => Processing Job: {}", job.id,);
                        let job_data = match job.data_json::<RequestPayload>() {
                            Ok(response) => response,
                            Err(_) => panic!("Could not process!"),
                        };

                        match job_data {
                            RequestPayload::PayloadProductDelete(payload) => {
                                println!("{} => Delete Job Action: {:?}", name, payload.action);
                            }
                            RequestPayload::PayloadProductSync(payload) => {
                                println!("{} => Sync Job Action: {:?}", name, payload.action);
                            }
                        }

                        sleep(PROCESSING_TIME).await;

                        work_queue
                            .complete(&mut conn, &job)
                            .await
                            .expect("Failed to mark a job as incomplete!");

                        println!("{name} => Completed Processing Job: {}", job.id);
                    }
                    None => {}
                }
            }
        }
        None => {}
    }
}
