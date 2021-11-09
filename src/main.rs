use crate::error::Error;
use crate::config::{Config, CovariancesConfig};
use crate::requests::{CovariancesRequest, MaskDefinition};
use crate::responses::Covariances;

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
    pub(crate) const MASK_GROUP_TYPE: &str = "REGION";
    pub(crate) const GROUP_NAME: &str = "the-group";
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    match Config::parse_config()? {
        Config::MetaData => { print_metadata().await }
        Config::Covariances(region) => { get_covariances(region).await? }
    }
    Ok(())
}

async fn print_metadata() {
    let resp = http::get_metadata().await;
    println!("{:#?}", resp);
}

async fn get_covariances(config: CovariancesConfig) -> Result<(), Error>{
    let mask_definition =
        MaskDefinition::new_single_group(
            constants::MASK_ID, String::from(constants::MASK_IDENTIFIER_TYPE),
            config.genome_build.clone(), String::from(constants::MASK_NAME),
            String::from(constants::MASK_DESCRIPTION),
            String::from(constants::MASK_GROUP_TYPE),
            String::from(constants::GROUP_NAME),
            config.region.start, config.region.stop
        );
    let request_data =
        CovariancesRequest::new(config.region, config.summary_statistic_dataset,
                                config.genome_build, vec![mask_definition]);
    let request_as_string = serde_json::to_string_pretty(&request_data)?;
    println!("{}", request_as_string);
    let response_res = http::get_covariances(request_data).await;
    match response_res {
        Ok(response) => {
            let data = &response.data;
            println!("{} variants, {} groups", &data.variants.len(), &data.groups.len());
            for group in &data.groups {
                println!("Group {}, {} variants, {} covariances", group.group,
                         group.variants.len(), group.covariance.len())
            }
            println!("{:#?}", &response);
        }
        Err(error) => {
            println!("{}", error)
        }
    }
    Ok(())
}

