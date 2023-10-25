use std::collections::HashMap;

use tracing::info;

use crate::{
    env_config::get_env_config,
    graphql::{GraphQLRequest, GraphQLResponse, GraphQLResponseData},
    http_client::{get_http_client, ContenType},
};

/*
 This function will fetch the category of a product from shopify.
*/
pub async fn get_shopify_product_category(product_graphql_id: String) -> String {
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

    // create variables
    let mut variables = HashMap::new();
    variables.insert("id".to_string(), product_graphql_id);

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

    category_name
}
