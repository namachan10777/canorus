mod preprocess;
mod parser;
mod analysis;
mod math;
mod backend;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub fn parse(s: &str) {
    let (_, data) = parser::parse(s).unwrap();
    let proc = analysis::Proc::new(&data);
    println!("{:#?}", proc);
    println!("{:#?}", backend::gen_gcode(proc));
}
