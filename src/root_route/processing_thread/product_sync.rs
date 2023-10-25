use redis_work_queue::Item;

use crate::{
    root_route::{processing_thread::category, processing_thread::upload_image},
    sanity::{
        http_payload::{Mutations, SanityMutationPayload},
        schema::{
            product::SanityProduct,
            root::{
                image::SanityCustomImage,
                link::SanityLink,
                seo::{SanitySEO, SanitySeoOpenGraph, SanitySlug},
            },
        },
    },
    shopify::PayloadProductSync,
};

pub async fn product_sync(
    name: &String,
    job: &Item,
    mutation_payload: &mut SanityMutationPayload,
    payload: PayloadProductSync,
) {
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
            upload_image::upload_image(product.images, product.title.clone(), &name, &job.id).await;

        /*
         * TODO: Category Stuff
         */

        category::category_check(admin_graphql_id.clone()).await;

        mutation_payload
            .mutations
            .push(Mutations::CreateOrReplace(SanityProduct {
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
            }))
    }
}
