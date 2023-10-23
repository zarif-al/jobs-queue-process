use serde::Deserialize;

// TODO: Update Sanity Response Handle
#[derive(Deserialize, Debug)]
pub struct SanityErrorResponse {
    pub description: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct SanityResponse {
    pub error: Option<SanityErrorResponse>,
}

#[derive(Deserialize, Debug)]
pub struct SanityDocument {
    pub _id: String,
}

#[derive(Deserialize, Debug)]
pub struct SanityResponseBody {
    pub document: SanityDocument,
}
