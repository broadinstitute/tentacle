use serde::Serialize;
use crate::region::Region;
use std::collections::HashMap;
use crate::constants;

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
    groups: HashMap<String, Group>,
}

#[derive(Serialize, Debug)]
pub(crate) struct Group {
    start: usize,
    stop: usize,
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
    pub(crate) fn new_single_group(region: Region, genome_build: String, dataset: usize)
        -> CovariancesRequest {
        let mask_definition =
            MaskDefinition::new_single_group(genome_build.clone(),
                                             region.start, region.stop);
        CovariancesRequest::new(region, dataset, genome_build,
                                vec![mask_definition])
    }
}

impl MaskDefinition {
    pub(crate) fn new(id: usize, identifier_type: String, genome_build: String, name: String,
                      description: String, group_type: String, groups: HashMap<String, Group>)
                      -> MaskDefinition {
        MaskDefinition {
            id,
            identifier_type,
            genome_build,
            name,
            description,
            group_type,
            groups,
        }
    }
    pub(crate) fn new_single_group(genome_build: String, start: usize, stop: usize)
                                   -> MaskDefinition {
        let group = Group::new(start, stop);
        let mut groups = HashMap::new();
        groups.insert(String::from(constants::GROUP_NAME), group);
        MaskDefinition::new(constants::MASK_ID,
                            String::from(constants::MASK_IDENTIFIER_TYPE),
                            genome_build, String::from(constants::MASK_NAME),
                            String::from(constants::MASK_DESCRIPTION),
                            String::from(constants::MASK_GROUP_TYPE), groups)
    }
}

impl Group {
    pub(crate) fn new(start: usize, stop: usize) -> Group {
        Group { start, stop }
    }
}