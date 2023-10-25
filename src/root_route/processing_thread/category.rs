use std::collections::HashMap;

use tracing::info;
use urlencoding::encode;

use crate::{
    env_config::get_env_config,
    graphql::{GraphQLRequest, GraphQLResponse, GraphQLResponseData},
    http_client::{get_http_client, ContenType},
    sanity::http_endpoint::{get_sanity_endpoint, ApiMode},
};

/*
 * TODO: Category Stuff
 * - Get the category of this product from shopify.
 * - Get the cateogry data from sanity
 *   - If category exists check if product is part of category
 *      - If yes, then continue
 *      - If no then create a mutation to push product to category.
 * - The mutation for category will be an option. So if some then trigger
 *   else move on.
 */
pub async fn category_check(product_graphql_id: String) {
    info!("Getting category info");

    let env_config = get_env_config();

    let client = get_http_client(ContenType::Data);

    // Create the request body
    let query = r#"
        query Product($id: ID!) {
            product(id: $id) {
                productCategory {
                    productTaxonomyNode {
                        id
                        name
                    }
                }
            }
        }"#;

    let mut variables = HashMap::new();
    variables.insert(
        "id".to_string(),
        "gid://shopify/Product/7841028800682".to_string(),
    );

    // create graphql request
    let graphql_request = GraphQLRequest {
        query: query.to_string(),
        variables,
    };

    // get product category from shopify
    let shopify_response = client
        .post(env_config.shopify_graphql_admin_api)
        .header("X-Shopify-Access-Token", env_config.shopify_admin_api_token)
        .json(&graphql_request)
        .send()
        .await
        .unwrap();

    let shopify_response_body = shopify_response.text().await.unwrap();

    let shopify_response_body_json: GraphQLResponse =
        serde_json::from_str(&shopify_response_body).unwrap();

    let category_name = match shopify_response_body_json.data {
        GraphQLResponseData::GraphQLShopifyProductQueryResponse(data) => {
            data.product.productCategory.productTaxonomyNode.name
        }
    };

    // check if sanity has category and product in the category
    let query = format!(
        "*[ _type == 'shopifyCategory' && title == '{}'][0]{{ items[] }}",
        category_name
    );

    // Percent-encode your query
    let encoded_query = encode(&query).to_string();

    let sanity_response = client
        .get(format!(
            "{}?query={}",
            get_sanity_endpoint(ApiMode::Query),
            encoded_query
        ))
        .send()
        .await
        .unwrap();

    // TODO : Type sanity response
    info!("Sanity Data : {:#?}", sanity_response.text().await.unwrap());
}
