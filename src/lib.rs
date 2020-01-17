use nom;
use std::fs;
use std::vec::Vec;

struct Parsed {
    desc: String,
    name: String,
    schema: String,
    data: Vec<Vec<Data>>,
}

struct Data {
    name: String,
    args: Vec<Arg>,
}

enum Arg {
    Str(String),
    Id(u64),
    Number(f64),
    Control(String),
}

pub fn parse(fs: &fs::File) -> String {
    String::new()
}
