use askama::Template;

#[derive(Template)]
#[template(path = "messages.html")]
pub struct MailTemplate {
    pub messages: Option<Vec<String>>,
}
