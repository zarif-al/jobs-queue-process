use crate::env_config::get_env_config;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use tracing::error;

pub fn get_mailer() -> Option<SmtpTransport> {
    let env_config = get_env_config();

    let creds = Credentials::new(env_config.gmail_email, env_config.gmail_app_password);

    match SmtpTransport::relay("smtp.gmail.com") {
        Ok(builder) => Some(builder.credentials(creds).build()),
        Err(err) => {
            error!("Failed to create transport builder. Error: {}", err);
            None
        }
    }
}
