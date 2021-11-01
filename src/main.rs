use crate::error::Error;
use crate::config::{Config, CovariancesConfig};
use crate::requests::CovariancesRequest;

mod responses;
mod error;
mod http;
mod config;
mod requests;
mod region;

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
    let request_data =
        CovariancesRequest::new(config.region, config.summary_statistic_dataset,
                                config.genome_build);
    let resp = http::get_covariances(request_data).await;
    println!("{:#?}", resp);
}

