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
                        .help("Sets a custom config file. If not given, it stores it in the users config file directory.")
                        .takes_value(true)
                    )
                    .arg(Arg::with_name("auto")
                        .short("a")
                        .long("auto")
                        .help("Automatically configures the displays according to the config file.")
                    )
                    .arg(Arg::with_name("default-profile")
                        .short("d")
                        .long("default-profile")
                        .value_name("PROFILE")
                        .help("Selects a profile to apply in case --auto does not recognize the current system config.")
                        .takes_value(true)
                    )
                    .arg(Arg::with_name("profile")
                        .short("p")
                        .long("profile")
                        .value_name("PROFILE")
                        .help("Applies the given profile.")
                        .takes_value(true)
                    )
                    .arg(Arg::with_name("save")
                        .short("s")
                        .long("save")
                        .help("Stores the current display configuration to the config file for a later automatic display configuration.")
                    )
                    .arg(Arg::with_name("create-empty")
                        .short("n")
                        .long("create-empty")
                        .help("Creates an empty config file.")
                    )
                    .arg(Arg::with_name("debug")
                        .long("debug")
                        .help("Does verbose printing, and only simulates calls to xrandr.")
                    )
                    .arg(Arg::with_name("info")
                        .short("i")
                        .long("info")
                        .help("Prints the contents of the config file and the current connected hardware in an abbreviated form.")
                    )
                    .get_matches();

    let debug = matches.is_present("debug");

    let config_path = if let Some(p) = matches.value_of_os("config") {
        p.into()
    } else {
        quickrandr::xdg_config_file().unwrap()
    };

    if matches.is_present("info") {
        quickrandr::cmd_info(&config_path, debug);
        return;
    }
    if matches.is_present("create-empty") {
        quickrandr::cmd_create_empty(&config_path, debug);
        return;
    }
    if let Some(profile) = matches.value_of("profile") {
        quickrandr::cmd_profile(&config_path, profile, debug);
        return;
    }
    if matches.is_present("auto") {
        let default_profile = matches.value_of("default-profile");

        quickrandr::cmd_auto(&config_path, default_profile, debug);
        return;
    }
    if matches.is_present("save") {
        quickrandr::cmd_save(&config_path, debug);
        return;
    }
}
