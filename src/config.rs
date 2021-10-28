use crate::error::Error;
use clap::{App, SubCommand, Arg};
use crate::region::Region;

pub(crate) enum Config {
    MetaData,
    Covariances(Region),
}

mod names {
    pub(crate) const METADATA_CMD: &str = "metadata";
    pub(crate) const COVARIANCES_CMD: &str = "covariances";
    pub(crate) const REGION_ARG: &str = "region";
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

            Ok(Config::Covariances(region))
        } else {
            Err(Error::from(
                format!("Need to provide subcommand '{}' or '{}.", names::METADATA_CMD,
                        names::COVARIANCES_CMD)
            ))
        }
    }
}