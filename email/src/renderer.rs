use crate::{EmailType, WelcomeEmail};

pub(crate) struct Renderer {}

impl Renderer {
    pub(crate) fn from(
        email_type: EmailType,
        tera: tera::Tera,
    ) -> Result<RenderedEmail, tera::Error> {
        match email_type {
            EmailType::Welcome(e) => Self::render_welcome(e, tera),
            EmailType::PasswordReset => todo!(),
            EmailType::NewLogin => todo!(),
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
}

#[derive(Debug)]
pub(crate) struct RenderedEmail {
    pub(crate) to: String,
    pub(crate) subject: String,
    pub(crate) body_html: String,
    pub(crate) body_text: String,
}
