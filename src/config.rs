use clap::Parser;

#[derive(Debug, Parser)]
pub struct ApiFiny {
    #[clap(required = true, env)]
    pub apifiny_access_key: String,
    #[clap(required = true, env)]
    pub apifiny_secret_key: String,
    #[clap(required = true, env)]
    pub apifiny_account_id: String,
}
