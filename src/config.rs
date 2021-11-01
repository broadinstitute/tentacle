use crate::error::Error;
use clap::{App, SubCommand, Arg};
use crate::region::Region;

pub(crate) enum Config {
    MetaData,
    Covariances(CovariancesConfig),
}

pub(crate) struct CovariancesConfig {
    pub(crate) region: Region,
    pub(crate) summary_statistic_dataset: usize,
    pub(crate) genome_build: String,
}

mod names {
    pub(crate) const METADATA_CMD: &str = "metadata";
    pub(crate) const COVARIANCES_CMD: &str = "covariances";
    pub(crate) const REGION_ARG: &str = "region";
    pub(crate) const DATASET_ARG: &str = "dataset";
    pub(crate) const GENOME_BUILD_ARG: &str = "genome_build";
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
                ).subcommand(SubCommand::with_name(names::COVARIANCES_CMD)
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
            let covariances_config =
                CovariancesConfig::new(region, dataset, genome_build);
                Ok(Config::Covariances(covariances_config))
        } else {
            Err(Error::from(
                format!("Need to provide subcommand '{}' or '{}.", names::METADATA_CMD,
                        names::COVARIANCES_CMD)
            ))
        }
    }
}

impl CovariancesConfig {
    fn new(region: Region, summary_statistic_dataset: usize, genome_build: String)
           -> CovariancesConfig {
        CovariancesConfig { region, summary_statistic_dataset, genome_build }
    }
}