mod preprocess;
mod parser;
mod analysis;
mod math;
mod backend;
pub mod license;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::result::Result;

pub type CNCConfig = backend::CNCConfig;

pub fn parse(s: &str, cfg: &CNCConfig) -> Result<(String, String), String> {
    let (_, data) = parser::parse(s).map_err(
        |e| match e {
            parser::ParseError::DataParseError(msg) => {
                format!("failed to parser data section: {}", msg)
            },
            parser::ParseError::HeaderParseError(msg) => {
                format!("failed to parser header section: {}", msg)
            },
            parser::ParseError::PreprocessError(msg) => {
                format!("failed to parser file. {}", msg)
            }
        }
    )?;
    let proc = analysis::Proc::new(&data);
    let report = proc.report.clone();
    let gcode = backend::gen_gcode(proc, cfg).map_err(|_| "internal error".to_owned())?;
    Ok((gcode, report))
}
