#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate xdg;

use std::io;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Xdg(xdg::BaseDirectoriesError),
}
impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error::Io(x)
    }
}
impl From<serde_json::Error> for Error {
    fn from(x: serde_json::Error) -> Self {
        Error::Json(x)
    }
}
impl From<xdg::BaseDirectoriesError> for Error {
    fn from(x: xdg::BaseDirectoriesError) -> Self {
        Error::Xdg(x)
    }
}

pub type DResult<T> = Result<T, Error>;

/// Shells out to xrandr and gets its `--verbose` output
pub fn query_xrandr() -> io::Result<String> {
    let mut child = Command::new("xrandr")
                                .arg("--verbose")
                                .stdout(Stdio::piped())
                                .spawn()?;

    let ecode = child.wait()?;
    assert!(ecode.success());

    let mut s = String::new();
    child.stdout.take().unwrap().read_to_string(&mut s)?;

    return Ok(s);
}

pub fn invoke_xrandr(args: &[String]) -> io::Result<()> {
    let mut child = Command::new("xrandr")
                                .args(args)
                                .spawn()?;

    let ecode = child.wait()?;
    assert!(ecode.success());

    return Ok(());
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Orientation {
    Normal,
    Left,
    Inverted,
    Right,
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub width: usize,
    pub height: usize,
    pub x_offset: usize,
    pub y_offset: usize,
    pub orientation: Orientation,
    pub is_primary: bool,
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub edid: String,
    pub geometry: Option<Geometry>
}

pub type ConnectedOutputs = HashMap<String, Output>;
pub type OutputDefaults = HashMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub configs: Vec<ConnectedOutputs>,
    pub default: OutputDefaults,
}

pub type OutputNames = Vec<String>;

impl Output {
    pub fn raw_edid_to_bytes(&self) -> Vec<u8> {
        let mut gather_bytes = Vec::new();

        for hex_byte in self.edid
            .as_bytes()
            .chunks(2)
            .map(|b| std::str::from_utf8(b).unwrap())
        {
            let byte = u8::from_str_radix(hex_byte, 16).unwrap();

            //println!("{}: {}", hex_byte, byte);
            gather_bytes.push(byte);
        }
        //println!("BYTES: {:?}", gather_bytes);
        gather_bytes
    }

    /*
    pub fn parse_edid(&self) -> EDID {
        edid::parse(&self.raw_edid_to_bytes()).unwrap().1
    }
    */
}


pub fn parse_xrandr(s: &str) -> (ConnectedOutputs, OutputNames) {
    let mut connected_outputs = HashMap::new();
    let mut output_names = Vec::new();

    let mut lines = s.lines();
    let mut line;

    macro_rules! unwrap_or_break {
        ($e:expr) => (
            if let Some(e) = $e {
                e
            } else {
                break;
            }
        )
    }

    macro_rules! next_line {
        ($line:expr, $lines:expr) => (
            if let Some(line) = $lines.next() {
                $line = line;
            } else {
                break;
            }
        )
    }

    // Parse away "Screen N" headers
    loop {
        line = lines.next().expect("Expected Displays");
        if !line.starts_with("Screen") {
            break;
        }
    }

    // Parse Displays
    loop {
        if line.starts_with(char::is_whitespace) {
            next_line!(line, lines);
            continue;
        }

        let mut splited = line.split_whitespace();
        let output_name = unwrap_or_break!(splited.next());
        let state = unwrap_or_break!(splited.next());

        if state.ends_with("connected") && !output_name.starts_with("VIRTUAL") {
            output_names.push(output_name.to_string());
        }

        if state != "connected" || output_name.starts_with("VIRTUAL") {
            next_line!(line, lines);
            continue;
        }

        let mut next = unwrap_or_break!(splited.next());
        let is_primary = next == "primary";
        if is_primary {
            next = splited.next().unwrap();
        }

        let is_part_of_desktop = next != "(normal";

        let mut geometry = None;
        if is_part_of_desktop {
            // parse current screen config and orientation
            let resolution = next;

            let mut iter = resolution.split(&['x', '+'][..]);
            let width = iter.next().unwrap().parse().unwrap();
            let height = iter.next().unwrap().parse().unwrap();
            let x_offset = iter.next().unwrap().parse().unwrap();
            let y_offset = iter.next().unwrap().parse().unwrap();

            let _unknown_hex_id = splited.next().unwrap();

            let orientation = splited.next().unwrap();
            let orientation = match orientation {
                "normal" => Orientation::Normal,
                "left" => Orientation::Left,
                "inverted" => Orientation::Inverted,
                "right" => Orientation::Right,
                _ => panic!("unknown orientation"),
            };

            geometry = Some(Geometry {
                orientation,
                width,
                height,
                x_offset,
                y_offset,
                is_primary,
            });
        }

        loop {
            next_line!(line, lines);

            assert!(line.starts_with(char::is_whitespace),
                    "no EDID Data found for connected device {}!",
                    output_name);

            if line.trim() == "EDID:" {
                let mut gather = String::new();
                for _ in 0..8 {
                    gather.push_str(lines.next().unwrap().trim());
                }

                let out = Output {
                    edid: gather,
                    geometry: geometry,
                };

                //println!("HEX: {}", out.edid);
                //println!("PARSED: {:?}", out.parse_edid());

                connected_outputs.insert(output_name.to_string(), out);

                break;
            }
        }

    }

    output_names.sort();
    (connected_outputs, output_names)
}

pub fn parse_json(s: &str) -> DResult<ConfigFile> {
    Ok(serde_json::from_str(s)?)
}

pub fn generate_json(p: &ConfigFile) -> DResult<String> {
    Ok(serde_json::to_string_pretty(p)?)
}

pub fn save_file(path: &Path, contents: &str) -> DResult<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(contents.as_bytes())?;
    buf_writer.get_ref().sync_all()?;

    Ok(())
}

pub fn load_file(path: &Path) -> DResult<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut ret = String::new();
    buf_reader.read_to_string(&mut ret)?;
    Ok(ret)
}

pub fn xdg_config_file() -> DResult<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("quickrandr")?;
    Ok(xdg_dirs.place_config_file("config.json")?)
}

pub struct ConfigAndXrandr {
    pub config_file: ConfigFile,
    pub connected_outputs: ConnectedOutputs,
    pub output_names: OutputNames,
}

pub fn load_config_and_query_xrandr(path: &Path) -> DResult<ConfigAndXrandr> {
    let config_file = {
        use std::thread;
        let path = path.to_owned();
        thread::spawn(move || parse_json(&load_file(&path)?))
    };
    let (connected_outputs, output_names) = parse_xrandr(&query_xrandr()?);
    let config_file = config_file.join().unwrap()?;

    Ok(ConfigAndXrandr {
        config_file,
        connected_outputs,
        output_names,
    })
}

pub fn save_config(path: &Path, config_file: &ConfigFile) -> DResult<()> {
    save_file(path, &generate_json(config_file)?)?;

    Ok(())
}

pub fn cmd_create_empty(path: &Path, debug: bool) {
    if fs::metadata(path).is_err() {
        let empty_database = ConfigFile {
            configs: Vec::new(),
            default: HashMap::new(),
        };

        let contents = generate_json(&empty_database).unwrap();

        if debug {
            println!("DEBUG: Write to path {:?}:\n{}", path.display(), contents);
        } else {
            save_file(path, &contents).unwrap();
        }
    }
}

pub fn fingerprint(connected_outputs: &ConnectedOutputs) -> Vec<(&str, &str)> {
    let mut fingerprint = connected_outputs
        .iter()
        .map(|(name, &Output { ref edid, .. })| (name.as_ref(), edid.as_ref()))
        .collect::<Vec<_>>();
    fingerprint.sort_by_key(|x| x.0);
    fingerprint
}

pub fn build_xrandr_args<F>(output_names: &[String], mut f: F) -> Vec<String>
    where F: FnMut(&str) -> Option<Vec<String>>
{
        let mut xrandr_command_queue = Vec::<String>::new();

        for output_name in output_names {
            xrandr_command_queue.push("--output".into());
            xrandr_command_queue.push(output_name.clone());

            if let Some(output_commands) = f(&output_name) {
                xrandr_command_queue.extend(output_commands);
            } else {
                xrandr_command_queue.push("--off".into());
            }
        }

        xrandr_command_queue
}

pub fn cmd_auto(path: &Path, debug: bool) {
    cmd_create_empty(path, debug);

    let ConfigAndXrandr {
        config_file,
        connected_outputs,
        output_names,
    } = load_config_and_query_xrandr(path).unwrap();

    let current_hardware_fingerprint = fingerprint(&connected_outputs);

    let xrandr_args;
    if let Some(target_config) = config_file.configs
        .iter().find(|x| fingerprint(x) == current_hardware_fingerprint)
    {
        // Found a fingerprint
        if debug {
            println!("FOUND target config: {:?}\n", target_config);
        }

        xrandr_args = build_xrandr_args(&output_names, |output_name| {
            let mut xrandr_command_queue = Vec::<String>::new();

            if let Some(geometry) = target_config
                .get(output_name)
                .and_then(|x| x.geometry.as_ref())
            {
                xrandr_command_queue.push("--mode".into());
                match geometry.orientation {
                    Orientation::Normal | Orientation::Inverted => {
                        xrandr_command_queue.push(format!("{}x{}", geometry.width, geometry.height));
                    }
                    Orientation::Left | Orientation::Right => {
                        xrandr_command_queue.push(format!("{}x{}", geometry.height, geometry.width));
                    }
                }

                xrandr_command_queue.push("--rotate".into());
                let orientation_str = match geometry.orientation {
                    Orientation::Normal => "normal",
                    Orientation::Inverted => "inverted",
                    Orientation::Left => "left",
                    Orientation::Right => "right",
                };
                xrandr_command_queue.push(orientation_str.into());

                xrandr_command_queue.push("--pos".into());
                xrandr_command_queue.push(format!("{}x{}", geometry.x_offset, geometry.y_offset));

                if geometry.is_primary {
                    xrandr_command_queue.push("--primary".into());
                }

                Some(xrandr_command_queue)
            } else {
                None
            }
        });
    } else {
        // Start working with defaults
        if debug {
            println!("DEFAULTS {:?}\n", config_file.default);
        }

        xrandr_args = build_xrandr_args(&output_names, |output_name| {
            if let Some(default) = config_file.default.get(output_name) {
                Some(default.split_whitespace().map(|x| x.to_string()).collect())
            } else {
                None
            }
        });
    }

    if debug {
        println!("xrandr args: {:?}", xrandr_args);
    } else {
        invoke_xrandr(&xrandr_args).unwrap();
    }

}

pub fn cmd_save(path: &Path, debug: bool) {
    cmd_create_empty(path, debug);

    let ConfigAndXrandr {
        mut config_file,
        connected_outputs,
        ..
    } = load_config_and_query_xrandr(path).unwrap();

    let mut found = false;
    {
        let current_hardware_fingerprint = fingerprint(&connected_outputs);
        if let Some(target_config) = config_file.configs
            .iter_mut().find(|x| fingerprint(x) == current_hardware_fingerprint)
        {
            *target_config = connected_outputs.clone();
            found = true;
        }
    }

    if !found {
        config_file.configs.push(connected_outputs.clone());
    }

    if debug {
        println!("Writing new config file:\n{}", generate_json(&config_file).unwrap());
    } else {
        save_config(path, &config_file).unwrap();
    }
}

pub fn cmd_list(path: &Path, debug: bool) {
    cmd_create_empty(path, debug);

    let ConfigAndXrandr {
        config_file,
        connected_outputs,
        ..
    } = load_config_and_query_xrandr(path).unwrap();

    let print_entry = |x: &ConnectedOutputs| {
        let mut v: Vec<_> = x.iter().collect();
        v.sort_by_key(|x| x.0);

        for x in v {
            print!("   {}:", x.0);
            if let Some(ref x) =  x.1.geometry {

                print!(" {:?}", x.orientation);

                print!(" {}x{}+{}+{}", x.width, x.height, x.x_offset, x.y_offset);

                if x.is_primary {
                    print!(" primary");
                }

            } else {
                print!(" disabled");
            }
            println!();
        }
    };

    println!("Config file:");
    for x in &config_file.configs {
        print_entry(x);
        println!();
    }
    println!("Current:");
    print_entry(&connected_outputs);
}

/*
    TODO:
    add some kind of override mechanism (force only notebook display)
*/
