extern crate xrandr_profile;
extern crate clap;

use std::path::Path;

use clap::{Arg, App};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .about(env!("CARGO_PKG_DESCRIPTION"))
                    .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file")
                        .required(true)
                        .takes_value(true))
                    .arg(Arg::with_name("auto")
                        .short("a")
                        .long("auto")
                        .help("Automatically configures the displays according to the config file"))
                    .arg(Arg::with_name("save")
                        .short("s")
                        .long("save")
                        .help("Stores the current display configuration to the config file for a later automatic display configuration."))
                    .arg(Arg::with_name("create-empty")
                        .long("create-empty")
                        .help("Creates a empty config file"))
                    .arg(Arg::with_name("debug")
                        .long("debug")
                        .help("Does verbose printing, and only simulates calls to xrandr"))
                    .get_matches();

    let config_path = matches.value_of_os("config").unwrap();
    let debug = matches.is_present("debug");

    if matches.is_present("create-empty") {
        xrandr_profile::cmd_create_empty(Path::new(&config_path), debug);
    }
    if matches.is_present("auto") {
        xrandr_profile::cmd_auto(Path::new(&config_path), debug);
    }
    if matches.is_present("save") {
        xrandr_profile::cmd_save(Path::new(&config_path), debug);
    }
}
