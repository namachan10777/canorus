mod preprocess;
mod parser;
mod analysis;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub fn parse(s: &str) {
    let (header, data) = parser::parse(s).unwrap();
    for d in data {
        println!("{:?}", d);
    }
}
