use crate::{EmailType, WelcomeEmail};

use super::types::{BirthdayEmail, NewLoginEmail, PasswordResetEmail};

pub(crate) struct Renderer {}

impl Renderer {
    pub(crate) fn from(
        email_type: EmailType,
        tera: tera::Tera,
    ) -> Result<RenderedEmail, tera::Error> {
        match email_type {
            EmailType::Welcome(e) => Self::render_welcome(e, tera),
            EmailType::PasswordReset(e) => Self::render_reset(e, tera),
            EmailType::NewLogin(e) => Self::render_newlogin(e, tera),
            EmailType::Birthday(e) => Self::render_birthday(e, tera),
        }
    }

    fn render_welcome(email: WelcomeEmail, tera: tera::Tera) -> Result<RenderedEmail, tera::Error> {
        let mut context = tera::Context::new();
        context.insert("name", &email.full_name);
        context.insert("id", &email.id);
        match tera.render("welcome", &context) {
            Ok(html) => Ok(RenderedEmail {
                to: email.to,
                subject: "Welcome to FriendHub!".to_owned(),
                body_html: html,
                body_text: "text".to_owned(),
            }),
            Err(e) => Err(e),
        }
    }

    fn render_reset(
        email: PasswordResetEmail,
        tera: tera::Tera,
    ) -> Result<RenderedEmail, tera::Error> {
        let mut context = tera::Context::new();
        context.insert("name", &email.full_name);
        context.insert("id", &email.id);
        match tera.render("password_reset", &context) {
            Ok(html) => Ok(RenderedEmail {
                to: email.to,
                subject: "Password reset request for your FriendHub account".to_owned(),
                body_html: html,
                body_text: "text".to_owned(),
            }),
            Err(e) => Err(e),
        }
    }

    fn render_newlogin(
        email: NewLoginEmail,
        tera: tera::Tera,
    ) -> Result<RenderedEmail, tera::Error> {
        let mut context = tera::Context::new();
        context.insert("name", &email.full_name);
        context.insert("country", &email.country);
        context.insert("region", &email.region);
        context.insert("city", &email.city);
        context.insert("isp", &email.isp);
        match tera.render("newlogin", &context) {
            Ok(html) => Ok(RenderedEmail {
                to: email.to,
                subject: "New device login to your FriendHub account".to_owned(),
                body_html: html,
                body_text: "text".to_owned(),
            }),
            Err(e) => Err(e),
        }
    }

    fn render_birthday(
        email: BirthdayEmail,
        tera: tera::Tera,
    ) -> Result<RenderedEmail, tera::Error> {
        let mut context = tera::Context::new();
        context.insert("name", &email.full_name);
        context.insert("id", &email.id);
        match tera.render("birthday", &context) {
            Ok(html) => Ok(RenderedEmail {
                to: email.to,
                subject: "Happy birthday from FriendHub!".to_owned(),
                body_html: html,
                body_text: "text".to_owned(),
            }),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub(crate) struct RenderedEmail {
    pub(crate) to: String,
    pub(crate) subject: String,
    pub(crate) body_html: String,
    pub(crate) body_text: String,
}
