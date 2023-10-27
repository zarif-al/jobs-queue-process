use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};

pub fn get_client() -> Client {
    let client_builder = reqwest::Client::builder();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = client_builder.default_headers(headers).build();

    client.unwrap()
}
