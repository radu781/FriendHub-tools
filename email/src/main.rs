mod emails;
mod exit_codes;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use emails::{
    renderer,
    types::{BirthdayEmail, NewLoginEmail, PasswordResetEmail, WelcomeEmail},
};
use exit_codes::{exit_with_info, Exit};
use renderer::Renderer;
use tera::Tera;

use crate::emails::sender::Sender;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    type_: EmailType,
}

#[derive(Subcommand)]
enum EmailType {
    Welcome(WelcomeEmail),
    PasswordReset(PasswordResetEmail),
    NewLogin(NewLoginEmail),
    Birthday(BirthdayEmail),
}

fn main() {
    match dotenv() {
        Ok(_) => {},
        Err(e) => exit_with_info(Exit::CannotLoadEnvironment(&e)),
    };
    let args = Cli::parse();
    let mut tera = Tera::default();

    if let Err(e) = tera.add_raw_templates(vec![
        ("welcome", include_str!("../templates/welcome.jinja2")),
        (
            "password_reset",
            include_str!("../templates/password_reset.jinja2"),
        ),
        ("newlogin", include_str!("../templates/new_login.jinja2")),
        ("birthday", include_str!("../templates/birthday.jinja2")),
    ]) {
        exit_with_info(Exit::TemplateLoadFailure(&e));
    }

    match Renderer::from(args.type_, tera) {
        Ok(file) => {
            Sender::send(file);
        }
        Err(e) => exit_with_info(Exit::RenderError(&e)),
    }
}
