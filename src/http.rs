use crate::responses::{MetaDataResponse, CovariancesResponse};
use crate::error::Error;
use crate::requests::CovariancesRequest;
use reqwest::Response;

pub(crate) async fn get_metadata() -> Result<MetaDataResponse, Error> {
    let metadata = reqwest::get("http://35.232.6.190/aggregation/metadata")
        .await?
        .json::<MetaDataResponse>()
        .await?;
    Ok(metadata)
}

async fn get_covariances_response(request_data: CovariancesRequest) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let response = client.post("http://35.232.6.190/aggregation/covariance")
        .json(&request_data)
        .send()
        .await?;
    Ok(response)
}

pub(crate) async fn get_covariances_parsed(request_data: CovariancesRequest)
                                           -> Result<CovariancesResponse, Error> {
    let response = get_covariances_response(request_data).await?;
    let http_status = response.status();
    if http_status.is_success() {
        let covariances = response.json::<CovariancesResponse>().await?;
        Ok(covariances)
    } else {
        let body = response.text().await?;
        let message = format!("HTTP status {}\n{}", http_status.as_str(), body);
        Err(Error::from(message))
    }
}

pub(crate) async fn get_covariances_raw(request_data: CovariancesRequest)
                                        -> Result<String, Error> {
    let response = get_covariances_response(request_data).await?;
    let http_status = response.status();
    if http_status.is_success() {
        let covariances = response.text().await?;
        Ok(covariances)
    } else {
        let body = response.text().await?;
        let message = format!("HTTP status {}\n{}", http_status.as_str(), body);
        Err(Error::from(message))
    }
}
