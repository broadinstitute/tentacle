use crate::error::Error;
use crate::config::Config;

mod responses;
mod error;
mod http;
mod config;
mod requests;
mod region;
mod variant;
mod get_covariances;
mod join_covariances;

mod constants {
    pub(crate) const MASK_ID: usize = 1;
    pub(crate) const MASK_IDENTIFIER_TYPE: &str = "COORDINATES";
    pub(crate) const MASK_NAME: &str = "Mask for just the region";
    pub(crate) const MASK_DESCRIPTION: &str = "Mask for just the region";
    pub(crate) const MASK_GROUP_TYPE: &str = "REGION";
    pub(crate) const GROUP_NAME: &str = "the-group";
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    match Config::parse_config()? {
        Config::MetaData => {
            print_metadata().await
        }
        Config::Covariances(covariances_config) => {
            get_covariances::get_covariances(covariances_config).await?
        }
        Config::JoinCovariances(join_covariances_config) => {
            join_covariances::join_covariances(join_covariances_config).await?
        }
    }
    Ok(())
}

async fn print_metadata() {
    let resp = http::get_metadata().await;
    println!("{:#?}", resp);
}
