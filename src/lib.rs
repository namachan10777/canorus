use std::fs;
pub mod parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

struct header {
    description: String,
    implementation_level: String,
    name: String,
    author: String,
    organization: String,
    preprocessor_version: String,
    originating_system: String,
    authorisation: String,
    file_schema: String,
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

pub fn parse(s : &str) -> Vec<advanced_face> {
    Vec::new()
}
