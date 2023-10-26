#![allow(non_snake_case)]

use serde::Serialize;

use crate::sanity::schema::root::image::SanityCustomImage;

use super::link::SanityLink;

#[derive(Serialize, Debug)]
pub struct SanitySlug {
    pub current: String,
}

#[derive(Serialize, Debug)]
pub struct SanitySeoOpenGraph {
    pub image: Option<SanityCustomImage>,
}

#[derive(Serialize, Debug)]
pub struct SanitySEO {
    pub title: String,
    pub slug: SanitySlug,
    pub description: String,
    pub disableIndex: bool,
    pub disableFollow: bool,
    pub disableImageIndex: bool,
    pub disableSnippet: bool,
    pub canonicalLink: SanityLink,
    pub openGraph: SanitySeoOpenGraph,
}
