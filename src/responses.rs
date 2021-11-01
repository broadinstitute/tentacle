use serde::Deserialize;
use std::collections::HashMap;
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetaData {
    data: Vec<MetaDataItem>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Covariances {
    data: CovariancesData
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetaDataItem {
    description: String,
    genome_build: String,
    masks: Vec<String>,
    name: String,
    summary_statistic_dataset: u32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CovariancesData {
    #[serde(flatten)]
    extra: HashMap<String, Value>
}
