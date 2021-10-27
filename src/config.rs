use crate::error::Error;
use clap::{App, SubCommand};

pub(crate) enum Config {
    MetaData
}

mod names {
    pub(crate) const METADATA_CMD: &str = "metadata";
}

impl Config {
    pub(crate) fn parse_config() -> Result<Config, Error> {
        let app =
            App::new(clap::crate_name!())
                .author(clap::crate_authors!())
                .version(clap::crate_version!())
                .about(clap::crate_description!())
                .subcommand(SubCommand::with_name(names::METADATA_CMD)
                    .help("Reads region from block-gzipped, tabix-index, location-sorted TSV file")
                );
        let matches = app.get_matches();
        if matches.subcommand_matches(names::METADATA_CMD).is_some() {
            Ok(Config::MetaData)
        } else {
            Err(Error::from(
                format!("Need to provide subcommand '{}'.", names::METADATA_CMD)
            ))
        }
    }
}