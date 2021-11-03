use serde::Serialize;
use crate::region::Region;
use std::collections::HashMap;

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
    pub(crate) fn new_single_group(id: usize, identifier_type: String, genome_build: String,
                                   name: String, description: String, group_type: String,
                                   group_name: String, start: usize, stop: usize)
                                   -> MaskDefinition {
        let group = Group::new(start, stop);
        let mut groups = HashMap::new();
        groups.insert(group_name, group);
        MaskDefinition::new(id, identifier_type, genome_build, name, description, group_type,
                            groups)
    }
}

impl Group {
    pub(crate) fn new(start: usize, stop: usize) -> Group {
        Group { start, stop }
    }
}