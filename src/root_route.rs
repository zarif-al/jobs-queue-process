// TODO: Clean up overall code. Look up how to clean up `match` usage

use crate::env_config::get_env_config;
use crate::sanity::product_schema::SanityProduct;
use crate::sanity::utils::{get_url, ApiMode};
use crate::sanity::{Mutations, SanityMutationPayload, SanityResponse};
use crate::shopify_payload::RequestPayload;
use crate::{client::get_client, db_connect};

use axum::{extract::Json, http::StatusCode};
use redis_work_queue::{Item, WorkQueue};
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};

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
            info!("{} => Ready to receive jobs!", name);

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

pub async fn processing_thread(name: String, work_queue: Arc<WorkQueue>) {
    let env_config = get_env_config();
    let mut mutation_payload: SanityMutationPayload = SanityMutationPayload {
        mutations: Vec::new(),
    };

    match db_connect::redis_conn().await {
        Some(mut conn) => {
            info!("{} => Ready to process jobs!", name);

            loop {
                let job: Option<Item> = work_queue
                    .lease(&mut conn, None, Duration::from_secs(60))
                    .await
                    .expect("Failed to lease a job!");

                match job {
                    Some(job) => {
                        info!("{} => Processing Job: {}", name, job.id,);
                        let job_data = match job.data_json::<RequestPayload>() {
                            Ok(response) => response,
                            Err(_) => panic!("Could not process!"),
                        };

                        match job_data {
                            RequestPayload::PayloadProductSync(payload) => {
                                info!("{} => Sync Job Action: {:?}", name, payload.action);

                                for product in payload.products {
                                    mutation_payload.mutations.push(Mutations::CreateOrReplace(
                                        SanityProduct {
                                            title: product.title,
                                            _type: "shopifyProduct".to_string(),
                                            shopify_id: product.id,
                                        },
                                    ))
                                }
                            }
                            RequestPayload::PayloadProductDelete(payload) => {
                                info!("{} => Delete Job Action: {:?}", name, payload.action);
                            }
                        }

                        let client = get_client();

                        let response = client
                            .post(get_url(ApiMode::Mutate))
                            .bearer_auth(&env_config.sanity_auth_token)
                            .json(&mutation_payload)
                            .send()
                            .await;

                        // TODO: Update error handle
                        match response {
                            Ok(resp) => {
                                if resp.status() != 200 {
                                    let res = resp.json::<SanityResponse>().await.unwrap();

                                    error!(
                                        "{} => Failed to complete Processing Job: {}",
                                        name, job.id
                                    );

                                    error!("{} => Error Message: {:?}", name, res.error.unwrap());
                                } else {
                                    work_queue
                                        .complete(&mut conn, &job)
                                        .await
                                        .expect("Failed to mark a job as incomplete!");

                                    info!("{} => Completed Processing Job: {}", name, job.id);
                                }
                            }
                            Err(err) => {
                                error!("{name} => Failed to complete Processing Job: {}", job.id);
                                error!("{name} => Error Message: {}", err)
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }
}
