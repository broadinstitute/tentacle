use crate::error::Error;
use crate::config::{Config, CovariancesConfig};
use crate::requests::{CovariancesRequest, MaskDefinition};

mod responses;
mod error;
mod http;
mod config;
mod requests;
mod region;

mod constants {
    pub(crate) const MASK_ID: usize = 1;
    pub(crate) const MASK_IDENTIFIER_TYPE: &str = "COORDINATES";
    pub(crate) const MASK_NAME: &str = "Mask for just the region";
    pub(crate) const MASK_DESCRIPTION: &str = "Mask for just the region";
    pub(crate) const MASK_GROUP_TYPE: &str = "Region";
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    match Config::parse_config()? {
        Config::MetaData => { print_metadata().await }
        Config::Covariances(region) => { get_covariances(region).await }
    }
    Ok(())
}

async fn print_metadata() {
    let resp = http::get_metadata().await;
    println!("{:#?}", resp);
}

async fn get_covariances(config: CovariancesConfig) {
    let mask_definition =
        MaskDefinition::new(
            constants::MASK_ID, String::from(constants::MASK_IDENTIFIER_TYPE),
            config.genome_build.clone(), String::from(constants::MASK_NAME),
            String::from(constants::MASK_DESCRIPTION),
            String::from(constants::MASK_GROUP_TYPE)
        );
    let request_data =
        CovariancesRequest::new(config.region, config.summary_statistic_dataset,
                                config.genome_build, vec![mask_definition]);
    let resp = http::get_covariances_text(request_data).await;
    println!("{:#?}", resp);
}

