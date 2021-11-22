use crate::responses::{MetaDataResponse, CovariancesResponse, CovariancesData};
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

async fn get_covariances_response(request: CovariancesRequest) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let response = client.post("http://35.232.6.190/aggregation/covariance")
        .json(&request)
        .send()
        .await?;
    Ok(response)
}

pub(crate) async fn get_covariances(request: CovariancesRequest)
                                    -> Result<CovariancesResponse, Error> {
    let response = get_covariances_response(request).await?;
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

pub(crate) async fn get_covariances_raw(request: CovariancesRequest)
                                        -> Result<String, Error> {
    let response = get_covariances_response(request).await?;
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
pub(crate) async fn get_covariance_data(request: CovariancesRequest)
    -> Result<CovariancesData, Error> {
    Ok(get_covariances(request).await?.data)
}
