use canorus;

use std::fs;
use clap;

fn main() {
    let matches = clap::App::new("canorus")
        .version("0.0.1")
        .author("Nakano Masaki <namachan10777@gmail.com>")
        .arg(clap::Arg::with_name("INPUT")
             .help("STEP file including only one rectangle lumber")
             .required(true)
             .index(1))
        .get_matches();
    println!("{:?}", &canorus::parse(&fs::File::open(matches.value_of("INPUT").unwrap()).unwrap()));
}
