use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};

#[derive(PartialEq)]
pub enum ContenType {
    Data,
    Image,
}

pub fn get_http_client(content_type: ContenType) -> Client {
    let client_builder = reqwest::Client::builder();

    let mut headers = HeaderMap::new();

    // TODO : Check if we need this
    if content_type == ContenType::Data {
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    } else {
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));
    }

    let client = client_builder.default_headers(headers).build();

    client.unwrap()
}
