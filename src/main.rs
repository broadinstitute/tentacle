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
mod constants;
mod find_common;

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
