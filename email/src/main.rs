mod exit_codes;
mod renderer;
mod sender;

use clap::{Args, Parser, Subcommand};
use dotenv::dotenv;
use exit_codes::{exit_with_info, Exit};
use renderer::Renderer;
use tera::Tera;

use crate::sender::Sender;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    type_: EmailType,
}

#[derive(Subcommand)]
enum EmailType {
    Welcome(WelcomeEmail),

    PasswordReset,

    NewLogin,
}

#[derive(Args)]
struct WelcomeEmail {
    #[arg(long)]
    full_name: String,

    #[arg(long)]
    id: String,

    #[arg(long)]
    to: String,
}

fn main() {
    dotenv().ok();
    let args = Cli::parse();
    let mut tera = Tera::default();

    if let Err(e) = tera.add_raw_templates(vec![
        ("welcome", include_str!("../templates/welcome.jinja2")),
        (
            "password_reset",
            include_str!("../templates/password_reset.jinja2"),
        ),
    ]) {
        exit_with_info(Exit::TemplateLoadFailure(&e));
    }

    match Renderer::from(args.type_, tera) {
        Ok(file) => {
            println!("{:?}", file);
            Sender::send(file);
        }
        Err(e) => exit_with_info(Exit::RenderError(&e)),
    }
}
