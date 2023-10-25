use tracing::info;
use urlencoding::encode;

use crate::{
    http_client::{get_http_client, ContenType},
    sanity::{
        http_endpoint::{get_sanity_endpoint, ApiMode},
        schema::category::SanityCategory,
    },
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SanityCategoryQueryResponse {
    pub result: SanityCategory,
}

/*
This function will sync the category and product with sanity.
*/
pub async fn sync_sanity_category(category_id: String) {
    let client = get_http_client(ContenType::Data);

    // check if sanity has category and product in the category
    let query = format!(
        "*[ _type == 'shopifyCategory' && title == '{}'][0]{{ items[] }}",
        category_id
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

    let sanity_response_body = sanity_response.text().await.unwrap();

    let sanity_response_body_json: SanityCategoryQueryResponse =
        serde_json::from_str(&sanity_response_body).unwrap();

    // TODO : Type sanity response
    info!("Sanity Data : {:#?}", sanity_response_body_json);
}
