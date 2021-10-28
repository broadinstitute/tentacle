use crate::error::Error;
use crate::config::Config;
use crate::region::Region;
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

async fn get_covariances(region: Region) {
    let request_data = CovariancesRequest::new(region);
    let resp = http::get_covariances(request_data).await;
    println!("{:#?}", resp);
}

