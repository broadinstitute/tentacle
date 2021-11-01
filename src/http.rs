use crate::responses::{MetaData, Covariances};
use crate::error::Error;
use crate::requests::CovariancesRequest;

pub(crate) async fn get_metadata() -> Result<MetaData, Error> {
    let metadata = reqwest::get("http://35.232.6.190/aggregation/metadata")
        .await?
        .json::<MetaData>()
        .await?;
    Ok(metadata)
}

pub(crate) async fn get_covariances(request_data: CovariancesRequest) -> Result<Covariances, Error> {
    let client = reqwest::Client::new();
    let covariances = client.post("http://35.232.6.190/aggregation/covariance")
        .json(&request_data)
        .send()
        .await?
        .json::<Covariances>()
        .await?;
    Ok(covariances)
}

pub(crate) async fn get_covariances_text(request_data: CovariancesRequest)
    -> Result<String, Error> {
    let client = reqwest::Client::new();
    let covariances = client.post("http://35.232.6.190/aggregation/covariance")
        .json(&request_data)
        .send()
        .await?
        .text()
        .await?;
    Ok(covariances)
}

