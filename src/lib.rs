use std::fs;
mod parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

pub fn parse(fs: &fs::File) -> String {
    String::new()
}
