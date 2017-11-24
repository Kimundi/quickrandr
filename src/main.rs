extern crate quickrandr;
extern crate clap;

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
                        .help("Sets a custom config file, or uses a default one in the according to the XDG convention")
                        .takes_value(true)
                        )
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

    let debug = matches.is_present("debug");

    let config_path = if let Some(p) = matches.value_of_os("config") {
        p.into()
    } else {
        quickrandr::xdg_config_file().unwrap()
    };

    if matches.is_present("create-empty") {
        quickrandr::cmd_create_empty(&config_path, debug);
    }
    if matches.is_present("auto") {
        quickrandr::cmd_auto(&config_path, debug);
    }
    if matches.is_present("save") {
        quickrandr::cmd_save(&config_path, debug);
    }
}
