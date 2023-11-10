/*
This module contains all the code necessary to send emails.
*/
mod mail_template;
mod mailer;
mod send_mail;

pub use send_mail::send_mail;
