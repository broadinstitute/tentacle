use serde::Serialize;
use crate::region::Region;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CovariancesRequest {
    chrom: String,
    start: usize,
    stop: usize,
    summary_statistic_dataset: usize,
    genome_build: String,
    mask_definitions: Vec<MaskDefinition>,
}

#[derive(Serialize, Debug)]
pub(crate) struct MaskDefinition {
    id: usize,
    identifier_type: String,
    genome_build: String,
    name: String,
    description: String,
    group_type: String,
}

impl CovariancesRequest {
    pub(crate) fn new(region: Region, summary_statistic_dataset: usize, genome_build: String,
                      mask_definitions: Vec<MaskDefinition>)
                      -> CovariancesRequest {
        let chrom = region.chrom;
        let start = region.start;
        let stop = region.stop;
        CovariancesRequest {
            chrom,
            start,
            stop,
            summary_statistic_dataset,
            genome_build,
            mask_definitions,
        }
    }
}

impl MaskDefinition {
    pub(crate) fn new(id: usize, identifier_type: String, genome_build: String, name: String,
                      description: String, group_type: String) -> MaskDefinition {
        MaskDefinition {
            id,
            identifier_type,
            genome_build,
            name,
            description,
            group_type,
        }
    }
}