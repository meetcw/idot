extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_matches() -> ArgMatches<'static> {
    return App::new("idot")
        .version("0.1.0")
        .version_short("v")
        .author("bright")
        .about("Simple dotfiles manager")
        .arg(Arg::with_name("debug").long("debug").short("d").help("Show debug information").global(true))
        .arg(Arg::with_name("simulate").long("simulate").short("s").help("Don't make any filesystem changes").global(true))
        .arg(Arg::with_name("workspace").takes_value(true).default_value(".").help("The directory that stored dotfiles").global(true))
        .subcommand(SubCommand::with_name("status")
            .version_short("v")
            .about("Show symbolic links status")
            .display_order(1))
        .subcommand(SubCommand::with_name("create")
            .version_short("v")
            .about("Create symbolic links by configuration")
            .display_order(2)
            .arg(Arg::with_name("force").long("force").short("f").help("Force to create symbolic link")))
        .subcommand(SubCommand::with_name("delete")
            .version_short("v")
            .display_order(3)
            .about("Delete symbolic links by configuration"))
        .get_matches();
}