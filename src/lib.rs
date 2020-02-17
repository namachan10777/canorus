mod preprocess;
mod parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::collections::HashMap;

pub fn parse(s: &str) {
    println!("{:?}", parser::parse(s));
}
