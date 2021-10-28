use serde::Serialize;
use crate::region::Region;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CovariancesRequest {
    chrom: String,
    start: usize,
    stop: usize
}

impl CovariancesRequest {
    pub(crate) fn new(region: Region) -> CovariancesRequest {
        let chrom = region.chrom;
        let start = region.start;
        let stop = region.stop;
        CovariancesRequest { chrom, start, stop }
    }
}