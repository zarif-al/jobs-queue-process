// TODO: Clean up overall code. Look up how to clean up `match` usage

use crate::env_config::get_env_config;
use crate::sanity::base::reference::SanityReference;
use crate::sanity::http_endpoint::{get_sanity_endpoint, ApiMode};
use crate::sanity::http_payload::{Mutations, SanityMutationPayload};
use crate::sanity::http_response::{SanityResponse, SanityResponseBody};
use crate::sanity::schema::product::SanityProduct;
use crate::sanity::schema::{
    root::image::{SanityCustomImage, SanityImage},
    root::link::SanityLink,
    root::seo::{SanitySEO, SanitySeoOpenGraph, SanitySlug},
};
use crate::shopify_payload::product_payload::ShopifyProductImage;
use crate::shopify_payload::RequestPayload;
use crate::{
    db_connect,
    http_client::{get_http_client, ContenType},
};

use axum::{extract::Json, http::StatusCode};
use nanoid::nanoid;
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
                                info!("{} => Sync Job Action: {:?}", name, payload.action);

                                for product in payload.products {
                                    let images: Vec<SanityCustomImage> = upload_image(
                                        product.images,
                                        product.title.clone(),
                                        &name,
                                        &job.id,
                                    )
                                    .await;

                                    let shopify_id: u64 = product
                                        .id
                                        .strip_prefix("gid://shopify/Product/")
                                        .unwrap()
                                        .parse()
                                        .unwrap();

                                    let admin_graphql_id: String = product.id;
                                    mutation_payload.mutations.push(Mutations::CreateOrReplace(
                                        SanityProduct {
                                            title: product.title.clone(),
                                            _type: "shopifyProduct".to_string(),
                                            shopify_id,
                                            admin_graphql_id,
                                            seo: SanitySEO {
                                                title: product.title.clone(),
                                                description: product.title.clone(),
                                                slug: SanitySlug {
                                                    current: product.handle,
                                                },
                                                canonicalLink: SanityLink { addLink: false },
                                                disableFollow: false,
                                                disableImageIndex: false,
                                                disableIndex: false,
                                                disableSnippet: false,
                                                openGraph: SanitySeoOpenGraph {
                                                    image: images.first().unwrap().clone(),
                                                },
                                            },
                                            shopify_created_at: product.createdAt,
                                            shopify_published_at: product.publishedAt,
                                            shopify_updated_at: product.updatedAt,
                                            status: product.status,
                                            tags: product.tags,
                                            product_type: product.productType,
                                            images,
                                        },
                                    ))
                                }
                            }
                            RequestPayload::PayloadProductDelete(payload) => {
                                info!("{} => Delete Job Action: {:?}", name, payload.action);
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

/*
    This function will upload an array of shopify images to sanity and
    return a vector of sanity images.

    TODO: There are many points of failures in this function, please address
    them appropriately.
*/
async fn upload_image(
    shopify_product_images: Vec<ShopifyProductImage>,
    product_title: String,
    thread_name: &String,
    job_id: &String,
) -> Vec<SanityCustomImage> {
    let env_config = get_env_config();

    let mut images: Vec<SanityCustomImage> = vec![];

    let client = get_http_client(ContenType::Image);

    info!(
        "{} => Uploading images for job: {}, Image count: {}",
        thread_name,
        job_id,
        shopify_product_images.len()
    );

    for image in shopify_product_images {
        // Download image bytes
        let featured_image = reqwest::get(image.src).await.unwrap();

        // Push image to sanity dataset.
        let image_response = client
            .post(get_sanity_endpoint(ApiMode::Assets))
            .bearer_auth(&env_config.sanity_auth_token)
            .body(featured_image)
            .send()
            .await
            .unwrap();

        match image_response.error_for_status_ref() {
            Ok(_) => {
                let image = image_response.text().await.unwrap();

                let image_json: SanityResponseBody = serde_json::from_str(&image).unwrap();

                images.push(SanityCustomImage {
                    alt: Some(product_title.clone()),
                    imageSrc: SanityImage {
                        _type: "image".to_string(),
                        asset: SanityReference {
                            _ref: image_json.document._id,
                            _type: "reference".to_string(),
                        },
                    },
                    _key: Some(nanoid!()),
                })
            }
            Err(err) => {
                let res = image_response.json::<SanityResponse>().await.unwrap();

                error!(
                    "{} => Failed to upload image. Error: {}. Error Message: {:?}",
                    thread_name,
                    err,
                    res.error.unwrap()
                );
                panic!();
            }
        }
    }

    return images;
}
