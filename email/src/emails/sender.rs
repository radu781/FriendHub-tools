use crate::exit_codes::{exit_with_info, Exit};
use crate::renderer::RenderedEmail;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub(crate) struct Sender {}

impl Sender {
    pub(crate) fn send(rendered_email: RenderedEmail) {
        let msg = match Message::builder()
            .from(
                format!("FriendHub mail <{}>", env::var("sender_email").unwrap())
                    .parse()
                    .unwrap(),
            )
            .to(format!("<{}>", rendered_email.to).parse().unwrap())
            .subject(rendered_email.subject)
            .header(ContentType::TEXT_HTML)
            .body(rendered_email.body_html)
        {
            Ok(m) => m,
            Err(e) => exit_with_info(Exit::CannotCreateMessageBuilder(&e)),
        };
        let creds = Credentials::new(
            env::var("sender_email").unwrap(),
            env::var("sender_password").unwrap(),
        );
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap_or_else(|e| exit_with_info(Exit::SmtpRelay(&e)))
            .credentials(creds)
            .build();
        match mailer.send(&msg) {
            Ok(_) => exit_with_info(Exit::Ok),
            Err(e) => exit_with_info(Exit::CannotSendEmail(&e)),
        }
    }
}
