use crate::error::Error;
use clap::{App, SubCommand, Arg};
use crate::region::Region;

pub(crate) enum Config {
    MetaData,
    Covariances(CovariancesConfig),
    JoinCovariances(JoinCovariancesConfig),
}

pub(crate) struct CovariancesConfig {
    pub(crate) region: Region,
    pub(crate) dataset: usize,
    pub(crate) genome_build: String,
    pub(crate) raw: bool,
}

pub(crate) struct JoinCovariancesConfig {
    pub(crate) region: Region,
    pub(crate) dataset1: usize,
    pub(crate) dataset2: usize,
    pub(crate) genome_build: String,
    pub(crate) out: String,
    pub(crate) dry: bool,
}

mod names {
    pub(crate) const METADATA_CMD: &str = "metadata";
    pub(crate) const COVARIANCES_CMD: &str = "covariances";
    pub(crate) const JOIN_COVARIANCES_CMD: &str = "join_covariances";
    pub(crate) const REGION_ARG: &str = "region";
    pub(crate) const DATASET_ARG: &str = "dataset";
    pub(crate) const DATASET_ARG1: &str = "dataset1";
    pub(crate) const DATASET_ARG2: &str = "dataset2";
    pub(crate) const GENOME_BUILD_ARG: &str = "genome_build";
    pub(crate) const OUT_ARG: &str = "out";
    pub(crate) const RAW_ARG: &str = "raw";
    pub(crate) const DRY_ARG: &str = "dry";
}

impl Config {
    pub(crate) fn parse_config() -> Result<Config, Error> {
        let app =
            App::new(clap::crate_name!())
                .author(clap::crate_authors!())
                .version(clap::crate_version!())
                .about(clap::crate_description!())
                .subcommand(SubCommand::with_name(names::METADATA_CMD)
                    .help("Get metadata.")
                )
                .subcommand(SubCommand::with_name(names::COVARIANCES_CMD)
                    .help("Get covariances")
                    .arg(Arg::with_name(names::REGION_ARG)
                        .long(names::REGION_ARG)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::REGION_ARG)
                    )
                    .arg(Arg::with_name(names::DATASET_ARG)
                        .long(names::DATASET_ARG)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::DATASET_ARG)
                    )
                    .arg(Arg::with_name(names::GENOME_BUILD_ARG)
                        .long(names::GENOME_BUILD_ARG)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::GENOME_BUILD_ARG)
                    )
                    .arg(Arg::with_name(names::RAW_ARG)
                        .long(names::RAW_ARG)
                    )
                )
                .subcommand(SubCommand::with_name(names::JOIN_COVARIANCES_CMD)
                    .help("Join covariances form two datasets into a file.")
                    .arg(Arg::with_name(names::REGION_ARG)
                        .long(names::REGION_ARG)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::REGION_ARG)
                    )
                    .arg(Arg::with_name(names::DATASET_ARG1)
                        .long(names::DATASET_ARG1)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::DATASET_ARG1)
                    )
                    .arg(Arg::with_name(names::DATASET_ARG2)
                        .long(names::DATASET_ARG2)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::DATASET_ARG2)
                    )
                    .arg(Arg::with_name(names::GENOME_BUILD_ARG)
                        .long(names::GENOME_BUILD_ARG)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::GENOME_BUILD_ARG)
                    )
                    .arg(Arg::with_name(names::OUT_ARG)
                        .long(names::OUT_ARG)
                        .required(true)
                        .takes_value(true)
                        .value_name(names::OUT_ARG)
                    )
                    .arg(Arg::with_name(names::DRY_ARG)
                        .long(names::DRY_ARG)
                        .required(false)
                        .takes_value(false)
                    )
                );
        let matches = app.get_matches();
        if matches.subcommand_matches(names::METADATA_CMD).is_some() {
            Ok(Config::MetaData)
        } else if let Some(covariance_matches) =
        matches.subcommand_matches(names::COVARIANCES_CMD) {
            let region =
                Region::parse(covariance_matches.value_of(names::REGION_ARG)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::REGION_ARG)
                    ))?)?;
            let dataset =
                str::parse::<usize>(covariance_matches.value_of(names::DATASET_ARG)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::DATASET_ARG)
                    ))?)?;
            let genome_build =
                String::from(covariance_matches.value_of(names::GENOME_BUILD_ARG)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::GENOME_BUILD_ARG)
                    ))?);
            let raw = covariance_matches.is_present(names::RAW_ARG);
            let covariances_config =
                CovariancesConfig::new(region, dataset, genome_build, raw);
            Ok(Config::Covariances(covariances_config))
        } else if let Some(join_covariance_matches) =
        matches.subcommand_matches(names::JOIN_COVARIANCES_CMD) {
            let region =
                Region::parse(join_covariance_matches.value_of(names::REGION_ARG)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::REGION_ARG)
                    ))?)?;
            let dataset1 =
                str::parse::<usize>(join_covariance_matches.value_of(names::DATASET_ARG1)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::DATASET_ARG1)
                    ))?)?;
            let dataset2 =
                str::parse::<usize>(join_covariance_matches.value_of(names::DATASET_ARG2)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::DATASET_ARG2)
                    ))?)?;
            let genome_build =
                String::from(join_covariance_matches.value_of(names::GENOME_BUILD_ARG)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::GENOME_BUILD_ARG)
                    ))?);
            let out =
                String::from(join_covariance_matches.value_of(names::GENOME_BUILD_ARG)
                    .ok_or_else(|| Error::from(
                        format!("Missing argument '{}'.", names::GENOME_BUILD_ARG)
                    ))?);
            let dry = join_covariance_matches.is_present(names::DRY_ARG);
            let join_covariances_config =
                JoinCovariancesConfig::new(region, dataset1, dataset2, genome_build, out, dry);
            Ok(Config::JoinCovariances(join_covariances_config))
        } else {
            Err(Error::from(
                format!("Need to provide subcommand '{}', '{}' or '{}.",
                        names::METADATA_CMD, names::COVARIANCES_CMD, names::JOIN_COVARIANCES_CMD)
            ))
        }
    }
}

impl CovariancesConfig {
    fn new(region: Region, dataset: usize, genome_build: String, raw: bool)
           -> CovariancesConfig {
        CovariancesConfig { region, dataset, genome_build, raw }
    }
}

impl JoinCovariancesConfig {
    fn new(region: Region, dataset1: usize, dataset2: usize, genome_build: String, out: String,
           dry: bool)
           -> JoinCovariancesConfig {
        JoinCovariancesConfig { region, dataset1, dataset2, genome_build, out, dry }
    }
}