extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
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

pub type Outputs = HashMap<String, Output>;
pub type OutputDefaults = HashMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    pub configs: Vec<Outputs>,
    pub default: OutputDefaults,
}

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


pub fn parse_xrandr(s: &str) -> Outputs {
    let mut parsed = HashMap::new();

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

        if state != "connected" {
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

                parsed.insert(output_name.to_string(), out);

                break;
            }
        }

    }

    parsed
}

pub fn parse_json(s: &str) -> DResult<Database> {
    Ok(serde_json::from_str(s)?)
}

pub fn generate_json(p: &Database) -> DResult<String> {
    Ok(serde_json::to_string(p)?)
}
