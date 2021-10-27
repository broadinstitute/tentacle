use crate::error::Error;
use crate::config::Config;

mod responses;
mod error;
mod http;
mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match Config::parse_config()? {
        Config::MetaData => { print_metadata().await }
    }
    Ok(())
}

async fn print_metadata() {
    let resp = http::get_metadata().await;
    println!("{:#?}", resp);
}