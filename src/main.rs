use std::path::PathBuf;

use clap::ArgMatches;
use log::{self, debug, error};

use crate::application::get_matches;
use crate::configuration::*;
use crate::error::*;
use crate::linker::Linker;
use crate::path_extension::PathExtension;

mod application;
mod configuration;
mod error;
mod linker;
mod logger;
mod path_extension;

fn main() {
    let matches = get_matches();
    let debug = matches.occurrences_of("debug") > 0;
    if debug {
        logger::Logger::new(log::LevelFilter::Info)
            .with_target_level("idot", log::LevelFilter::max())
            .init();
    } else {
        logger::Logger::new(log::LevelFilter::Info)
            .with_target_level("idot", log::LevelFilter::Info)
            .init();
    }
    if let Err(error) = handler(matches) {
        error!("{}", error);
    }
}


fn handler(matches: ArgMatches<'static>) -> Result<()> {
    debug!("args: {:?}", matches);
    let workspace = PathBuf::from(matches.value_of("workspace").unwrap_or("."))
        .absolutize()
        .map_err(|e| Error::new("Invalid workspace path").with_inner_error(&e))?;
    let simulate = matches.occurrences_of("simulate") > 0;
    let loader = DefaultGroupConfigurationLoader::new();
    match matches.subcommand() {
        ("status", Some(_matches)) => {
            let configuration = loader.load(&workspace)?;
            debug!("configuration: {:?}", configuration);
            Linker::status(&workspace, &configuration)
        }
        ("create", Some(matches)) => {
            let force = matches.occurrences_of("force") > 0;
            let mut configuration = loader.load(&workspace)?;
            if force {
                configuration.force = Some(force);
            }
            debug!("configuration: {:?}", configuration);
            Linker::create(&workspace, &configuration,simulate)
        }
        ("delete", Some(_matches)) => {
            let configuration = loader.load(&workspace)?;
            debug!("configuration: {:?}", configuration);
            Linker::delete(&workspace, &configuration,simulate)
        }
        _ => {
            let configuration = loader.load(&workspace)?;
            debug!("configuration: {:?}", configuration);
            Linker::status(&workspace, &configuration)
        }
    }
}
