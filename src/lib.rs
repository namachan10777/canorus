mod preprocess;
mod parser;
mod analysis;
mod math;
mod backend;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub type CNCConfig = backend::CNCConfig;

pub fn parse(s: &str, cfg: &CNCConfig) -> String {
    let (_, data) = parser::parse(s).unwrap();
    let proc = analysis::Proc::new(&data);
    backend::gen_gcode(proc, cfg).unwrap()
}
