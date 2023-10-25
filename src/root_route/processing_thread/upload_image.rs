use nanoid::nanoid;
use tracing::{error, info};

use crate::{
    env_config::get_env_config,
    http_client::{get_http_client, ContenType},
    sanity::{
        base::reference::SanityReference,
        http_endpoint::{get_sanity_endpoint, ApiMode},
        http_response::{SanityResponse, SanityResponseBody},
        schema::root::image::{SanityCustomImage, SanityImage},
    },
    shopify::product::ShopifyProductImage,
};

/*
    This function will upload an array of shopify images to sanity and
    return a vector of sanity images.

    TODO: There are many points of failures in this function, please address
    them appropriately.
*/
pub async fn upload_image(
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
