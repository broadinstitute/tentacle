use crate::config::JoinCovariancesConfig;
use crate::error::Error;
use crate::requests::CovariancesRequest;
use crate::http;
use crate::responses::{CovariancesData, Group};

pub(crate) async fn join_covariances(config: JoinCovariancesConfig)
                                     -> Result<(), Error> {
    let request1 =
        CovariancesRequest::new_single_group(config.region.clone(),
                                             config.genome_build.clone(),
                                             config.dataset1);
    let request2 =
        CovariancesRequest::new_single_group(config.region, config.genome_build,
                                             config.dataset2);
    let group1 =
        get_single_group(http::get_covariance_data(request1).await?)?;
    let group2 =
        get_single_group(http::get_covariance_data(request2).await?)?;

    todo!()
}

fn get_single_group(covariances_data: CovariancesData) -> Result<Group, Error> {
    let mut groups = covariances_data.groups;
    if groups.len() != 1 {
        Err(Error::from(
            format!("Expected exactly one group from data set {}, but got {} groups.",
                    covariances_data.summary_statistic_dataset, groups.len()))
        )
    } else {
        Ok(groups.remove(0))
    }
}