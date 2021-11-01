use serde::Serialize;
use crate::region::Region;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CovariancesRequest {
    chrom: String,
    start: usize,
    stop: usize,
    summary_statistic_dataset: usize,
    genome_build: String
}

impl CovariancesRequest {
    pub(crate) fn new(region: Region, summary_statistic_dataset: usize, genome_build: String)
        -> CovariancesRequest {
        let chrom = region.chrom;
        let start = region.start;
        let stop = region.stop;
        CovariancesRequest { chrom, start, stop, summary_statistic_dataset, genome_build }
    }
}