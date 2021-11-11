use serde::Deserialize;
use std::collections::HashMap;
use serde_json::value::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetaDataResponse {
    data: Vec<MetaData>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CovariancesResponse {
    pub(crate) data: CovariancesData
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetaData {
    pub(crate) description: String,
    pub(crate) genome_build: String,
    pub(crate) masks: Vec<String>,
    pub(crate) name: String,
    pub(crate) summary_statistic_dataset: u32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CovariancesData {
    pub(crate) summary_statistic_dataset: usize,
    pub(crate) variants: Vec<VariantData>,
    pub(crate) groups: Vec<Group>,
    pub(crate) n_samples: Option<f64>,
    pub(crate) sigma_squared: Option<f64>,
    #[serde(flatten)]
    pub(crate) extra: HashMap<String, Value>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariantData {
    pub(crate) alt_freq: f64,
    pub(crate) pvalue: f64,
    pub(crate) score: f64,
    pub(crate) variant: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Group {
    pub(crate) covariance: Vec<f64>,
    pub(crate) group: String,
    pub(crate) group_type: String,
    pub(crate) mask: usize,
    pub(crate) variants: Vec<String>
}