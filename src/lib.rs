use std::fs;
pub mod parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Default, Debug)]
pub struct header {
    description: Vec<String>,
    implementation_level: String,
    name: String,
    time_stamp: String,
    author: Vec<String>,
    organization: Vec<String>,
    preprocessor_version: String,
    originating_system: String,
    authorisation: Vec<String>,
    file_schema: Vec<String>,
}

type v3 = (f64, f64, f64);

#[derive(Debug)]
pub struct axis {
    p: v3,
    axis1: v3,
    axis2: v3,
}

#[derive(Debug)]
pub enum face_element {
    Cylinder(f64, axis),
    Plane(axis),
}

#[derive(Debug)]
pub struct advanced_face {
    flag: bool,
    elem: face_element,
}

pub fn parse(s : &str) -> (header, Vec<advanced_face>) {
    let parsed = parser::parse(s).unwrap();
    let mut header = header::default();
    for desc in parsed.header {
        match desc.name.as_str() {
            "FILE_DESCRIPTION" => {
                println!("{:?}", desc.args);
                header.description = desc.args[0].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
                header.implementation_level = desc.args[1].clone().str().unwrap();
            },
            "FILE_NAME" => {
                header.name = desc.args[0].clone().str().unwrap();
                header.time_stamp = desc.args[1].clone().str().unwrap();
                header.author = desc.args[2].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
                header.organization = desc.args[3].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
                header.preprocessor_version = desc.args[4].clone().str().unwrap();
                header.originating_system = desc.args[5].clone().str().unwrap();
                header.originating_system = desc.args[6].clone().str().unwrap();
            },
            "FILE_SCHEMA" => {
                header.file_schema = desc.args[0].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
            },
            _ => {
            }
        }
    }
    (header, Vec::new())
}
