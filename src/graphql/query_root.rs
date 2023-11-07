/*
This module contains all the queries for this application.
*/
use async_graphql::{Error, Object};
use serde_email::is_valid_email;

use crate::db::get_mongo_messages;
use crate::graphql::helper_structs::ResolvedMessageList;
use crate::mail::send_mail;

use crate::graphql::helper_structs::GeneralResponse;

pub struct GraphQLQueryRoot;

#[Object]
impl GraphQLQueryRoot {
    async fn get_messages(&self, email: String) -> Result<ResolvedMessageList, Error> {
        let is_valid_email = is_valid_email(&email);

        if is_valid_email {
            match get_mongo_messages(&email).await {
                Some(messages) => Ok(ResolvedMessageList { email, messages }),
                None => Err(Error {
                    message: String::from("Internal Server Error."),
                    source: None,
                    extensions: None,
                }),
            }
        } else {
            Err(Error {
                message: String::from("Invalid email"),
                source: None,
                extensions: None,
            })
        }
    }
    // TODO : Implement this
    async fn email_messages(&self, email: String) -> Result<GeneralResponse, Error> {
        let is_valid_email = is_valid_email(&email);

        if is_valid_email {
            match get_mongo_messages(&email).await {
                Some(messages) => match send_mail(&email, messages) {
                    Some(_) => Ok(GeneralResponse {
                        error: None,
                        message: Some("Please check your mail".to_string()),
                    }),
                    None => Ok(GeneralResponse {
                        error: Some("Failed to send mail".to_string()),
                        message: None,
                    }),
                },
                None => Err(Error {
                    message: String::from("Internal Server Error."),
                    source: None,
                    extensions: None,
                }),
            }
        } else {
            Err(Error {
                message: String::from("Invalid email"),
                source: None,
                extensions: None,
            })
        }
    }
}
