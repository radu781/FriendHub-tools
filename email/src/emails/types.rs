use clap::Args;

#[derive(Args)]
pub(crate) struct WelcomeEmail {
    #[arg(long)]
    pub(crate) full_name: String,

    #[arg(long)]
    pub(crate) id: String,

    #[arg(long)]
    pub(crate) to: String,
}

#[derive(Args)]
pub(crate) struct PasswordResetEmail {
    #[arg(long)]
    pub(crate) full_name: String,

    #[arg(long)]
    pub(crate) id: String,

    #[arg(long)]
    pub(crate) to: String,

    #[arg(long)]
    pub(crate) token: String,
}

#[derive(Args)]
pub(crate) struct NewLoginEmail {
    #[arg(long)]
    pub(crate) full_name: String,

    #[arg(long)]
    pub(crate) to: String,

    #[arg(long)]
    pub(crate) country: String,

    #[arg(long)]
    pub(crate) region: String,

    #[arg(long)]
    pub(crate) city: String,

    #[arg(long)]
    pub(crate) isp: String,
}

#[derive(Args)]
pub(crate) struct BirthdayEmail {
    #[arg(long)]
    pub(crate) full_name: String,

    #[arg(long)]
    pub(crate) id: String,

    #[arg(long)]
    pub(crate) to: String,
}
