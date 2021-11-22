use crate::config::CovariancesConfig;
use crate::error::Error;
use crate::requests::CovariancesRequest;
use crate::http;

pub(crate) async fn get_covariances(config: CovariancesConfig) -> Result<(), Error>{
    let request =
        CovariancesRequest::new_single_group(config.region, config.genome_build, config.dataset);
    let request_as_string = serde_json::to_string_pretty(&request)?;
    println!("{}", request_as_string);
    if !config.raw {
        let response_res = http::get_covariances(request).await;
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
        let response_res = http::get_covariances_raw(request).await;
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

