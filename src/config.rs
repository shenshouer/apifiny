use clap::Parser;

#[derive(Debug, Parser)]
pub struct ApiFiny {
    #[clap(required = true, env)]
    pub apifiny_api_key: String,
    #[clap(required = true, env)]
    pub apifiny_api_secret: String,
    #[clap(required = true, env)]
    pub apifiny_account_id: String,
}
