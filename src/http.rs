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
    let response = client.post("http://35.232.6.190/aggregation/covariance")
        .json(&request_data)
        .send()
        .await?;
    let http_status = response.status();
    if http_status.is_success() {
        let covariances = response.json::<Covariances>().await?;
        Ok(covariances)
    } else {
        let body = response.text().await?;
        let message = format!("HTTP status {}\n{}", http_status.as_str(), body);
        Err(Error::from(message))
    }
}
