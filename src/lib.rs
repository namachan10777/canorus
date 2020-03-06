mod preprocess;
mod parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub fn parse(s: &str) {
    println!("{:?}", parser::parse(s));
}
