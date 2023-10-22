#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SanityLink {
    pub addLink: bool,
}
