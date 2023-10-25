mod category;
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
        http_payload::{Mutations, SanityMutationPayload},
        http_response::SanityResponse,
        schema::{
            product::SanityProduct,
            root::{
                image::SanityCustomImage,
                link::SanityLink,
                seo::{SanitySEO, SanitySeoOpenGraph, SanitySlug},
            },
        },
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
                                info!("{} => Sync Job Action: {:?}", name, payload.action);

                                for product in payload.products {
                                    // separate shopify_id and graphql_id
                                    let shopify_id: u64 = product
                                        .id
                                        .strip_prefix("gid://shopify/Product/")
                                        .unwrap()
                                        .parse()
                                        .unwrap();

                                    let admin_graphql_id: String = product.id;

                                    // get product image references.
                                    let images: Vec<SanityCustomImage> =
                                        upload_image::upload_image(
                                            product.images,
                                            product.title.clone(),
                                            &name,
                                            &job.id,
                                        )
                                        .await;

                                    /*
                                     * TODO: Category Stuff
                                     */

                                    category::category_check(admin_graphql_id.clone()).await;

                                    mutation_payload.mutations.push(Mutations::CreateOrReplace(
                                        SanityProduct {
                                            _id: format!("shopifyProduct-{}", shopify_id),
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
