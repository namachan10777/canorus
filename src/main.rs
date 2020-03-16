use canorus;

use std::fs;
use std::io::{Read, Write};
use std::process;
use clap;

fn main() {
    let matches = clap::App::new("canorus")
        .version("0.0.1")
        .author("Nakano Masaki <namachan10777@gmail.com>")
        .arg(clap::Arg::with_name("INPUT")
            .help("STEP file including only one rectangle lumber")
            .required(true)
            .index(1))
        .arg(clap::Arg::with_name("CONFIG")
            .help("CNC configuration")
            .required(true)
            .short("c")
            .long("config")
            .takes_value(true))
        .arg(clap::Arg::with_name("OUTPUT")
            .help("output file")
            .required(false)
            .short("o")
            .long("output")
            .takes_value(true))
        .get_matches();
    let mut buf = String::new();
    let mut config_file = match fs::File::open(matches.value_of("CONFIG").unwrap()) {
        Ok(f) => f,
        Err(e) => {
            println!("Cannot open config file");
            println!("caused by {:?}", e);
            process::exit(-1)
        },
    };
    config_file.read_to_string(&mut buf).unwrap();
    let cfg: canorus::CNCConfig = serde_json::from_str(&buf).unwrap();
    buf.clear();

    let mut step_file = match fs::File::open(matches.value_of("INPUT").unwrap()) {
        Ok(f) => f,
        Err(e) => {
            println!("Cannot open step file");
            println!("caused by {:?}", e);
            process::exit(-1)
        },
    };
    step_file.read_to_string(&mut buf).unwrap();
    let gcode = canorus::parse(&buf, &cfg);
    match gcode {
        Ok(gcode) => {
            if let Some(output) = matches.value_of("OUTPUT") {
                let mut f = fs::File::create(output).unwrap();
                f.write_all(gcode.as_bytes()).unwrap();
            }
            else {
                println!("{}", gcode);
            }
        },
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
