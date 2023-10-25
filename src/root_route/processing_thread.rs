mod product_category;
mod product_sync;
mod upload_image;

use std::{sync::Arc, time::Duration};

use redis_work_queue::{Item, WorkQueue};
use tracing::{error, info};

use crate::{
    db_connect,
    env_config::get_env_config,
    http_client::{get_http_client, ContenType},
    sanity::{
        http_endpoint::{get_sanity_endpoint, ApiMode},
        http_payload::SanityMutationPayload,
        http_response::SanityResponse,
    },
    shopify::RequestPayload,
};

pub async fn processing_thread(name: String, work_queue: Arc<WorkQueue>) {
    let env_config = get_env_config();
    let mut mutation_payload: SanityMutationPayload = SanityMutationPayload {
        mutations: Vec::new(),
    };

    // get db connection
    match db_connect::redis_conn().await {
        Some(mut conn) => {
            info!("{} => Ready to process jobs!", name);

            // await to lease jobs from a queue.
            // run the job processs.
            // run in infinite loop.
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

                        let job_data = match job.data_json::<RequestPayload>() {
                            Ok(response) => response,
                            Err(_) => panic!("Could not process!"),
                        };

                        match job_data {
                            RequestPayload::PayloadProductSync(payload) => {
                                info!("{} => Commencing Sync Job Action!", name);
                                product_sync::product_sync(
                                    &name,
                                    &job,
                                    &mut mutation_payload,
                                    payload,
                                )
                                .await;
                            }
                            RequestPayload::PayloadProductDelete(payload) => {
                                info!("{} => Delete Job Action: {:?}", name, payload.action);
                            }
                            RequestPayload::PayloadCollectionsSync(payload) => {
                                todo!()
                            }
                            RequestPayload::PayloadCollectionsDelete(payload) => {
                                todo!()
                            }
                        }

                        let client = get_http_client(ContenType::Data);

                        let response = client
                            .post(get_sanity_endpoint(ApiMode::Mutate))
                            .bearer_auth(&env_config.sanity_auth_token)
                            .json(&mutation_payload)
                            .send()
                            .await;

                        // TODO: Update error handle
                        match response {
                            Ok(resp) => match resp.error_for_status_ref() {
                                Ok(_) => {
                                    work_queue
                                        .complete(&mut conn, &job)
                                        .await
                                        .expect("Failed to mark a job as incomplete!");

                                    info!("{} => Completed Processing Job: {}", name, job.id);
                                }
                                Err(err) => {
                                    let res = resp.json::<SanityResponse>().await.unwrap();

                                    error!("{} => Failed to process job: {}, Error Message: {}, Error Body: {:?}", name, job.id, err.without_url(), res.error.unwrap());
                                }
                            },
                            Err(err) => {
                                error!("{} =>  Failed to complete Processing Job: {}, Error Message: {}",name,job.id, err)
                            }
                        }
                    }
                    None => {
                        // No jobs have been found.
                        // We will continue looking for jobs.
                        continue;
                    }
                }
            }
        }
        None => {}
    }
}
