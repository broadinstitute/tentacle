use crate::config::CovariancesConfig;
use crate::error::Error;
use crate::requests::{MaskDefinition, CovariancesRequest};
use crate::{constants, http};

pub(crate) async fn get_covariances(config: CovariancesConfig) -> Result<(), Error>{
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
        CovariancesRequest::new(config.region, config.dataset,
                                config.genome_build, vec![mask_definition]);
    let request_as_string = serde_json::to_string_pretty(&request_data)?;
    println!("{}", request_as_string);
    if !config.raw {
        let response_res = http::get_covariances_parsed(request_data).await;
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
    } else {
        let response_res = http::get_covariances_raw(request_data).await;
        match response_res {
            Ok(response) => {
                println!("{}", response);
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }
    Ok(())
}

