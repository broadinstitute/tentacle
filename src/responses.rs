use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetaData {
    data: Vec<MetaDataItem>
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