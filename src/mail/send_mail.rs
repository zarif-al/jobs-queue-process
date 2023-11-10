use std::str::FromStr;

use lettre::{
    message::{Mailbox, SinglePart},
    Message, Transport,
};
use tracing::{error, info};

use crate::env_config::get_env_config;
use crate::mail::mail_template::MailTemplate;
use crate::mail::mailer::get_mailer;

pub fn send_mail(to_email: &String, messages: Vec<String>) -> Option<()> {
    let env_config = get_env_config();

    let from_email = format!("Zarif Mail Engine <{}>", env_config.gmail_email);
    let from_mbox = match Mailbox::from_str(&from_email) {
        Ok(mbox) => Some(mbox),
        Err(err) => {
            error!("Failed to create Mailbox from from_email. Error: {}", err);
            None
        }
    };

    let to_mbox = match Mailbox::from_str(&to_email) {
        Ok(mbox) => Some(mbox),
        Err(err) => {
            error!("Failed to create Mailbox from to_email. Error: {}", err);
            None
        }
    };

    if from_mbox.is_some() && to_mbox.is_some() {
        let mail;

        if messages.is_empty() {
            mail = MailTemplate { messages: None };
        } else {
            mail = MailTemplate {
                messages: Some(messages),
            };
        }

        let email = match Message::builder()
            .from(from_mbox.unwrap())
            .to(to_mbox.unwrap())
            .subject("Zarif Rust Job Processor: Message List")
            .singlepart(SinglePart::html(mail.to_string()))
        {
            Ok(email) => Some(email),
            Err(err) => {
                error!("Failed to build email. Error: {}", err);
                None
            }
        };

        match email {
            Some(email) => {
                let mailer = get_mailer();
                match mailer {
                    Some(mailer) => {
                        // Send the email
                        match mailer.send(&email) {
                            Ok(_) => {
                                info!("Email sent successfully!");
                                Some(())
                            }
                            Err(e) => {
                                error!("Could not send email: {e:?}");
                                None
                            }
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    } else {
        None
    }
}
