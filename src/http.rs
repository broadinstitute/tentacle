use crate::responses::MetaData;
use crate::error::Error;

pub(crate) async fn get_metadata() -> Result<MetaData, Error> {
    let metadata = reqwest::get("http://35.232.6.190/aggregation/metadata")
        .await?
        .json::<MetaData>()
        .await?;
    Ok(metadata)
}
