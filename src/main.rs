extern crate xrandr_profile;
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let x = xrandr_profile::query_xrandr().unwrap();
    let x = xrandr_profile::parse_xrandr(&x);

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
                    .arg(Arg::with_name("debug")
                        .long("debug")
                        .help("Does verbose printing, and only simulates calls to xrandr"))
                    .get_matches();


    println!("Current setup:");
    for (name, o) in x {
        println!("Display: {}, EDID: {}", name, o.edid);
        println!("    Geometry: {:?}", o.geometry);
    }
}
