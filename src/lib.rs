pub mod parser;
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Default, Debug)]
pub struct Header {
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

type V3 = (f64, f64, f64);

#[derive(Debug)]
pub struct Axis {
    p: V3,
    axis1: V3,
    axis2: V3,
}

#[derive(Debug)]
pub enum FaceElement {
    Cylinder(f64, Axis),
    Plane(Axis),
}

#[derive(Debug)]
pub struct AdvancedFace {
    flag: bool,
    elem: FaceElement,
}

fn parse_header(parsed_header: parser::Header) -> Header {
    let mut header = Header::default();
    for desc in parsed_header {
        match desc {
            parser::Value::Desc(name, args) =>
                match name.as_str() {
                    "FILE_DESCRIPTION" => {
                        println!("{:?}", args);
                        header.description = args[0].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
                        header.implementation_level = args[1].clone().str().unwrap();
                    },
                    "FILE_NAME" => {
                        header.name = args[0].clone().str().unwrap();
                        header.time_stamp = args[1].clone().str().unwrap();
                        header.author = args[2].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
                        header.organization = args[3].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
                        header.preprocessor_version = args[4].clone().str().unwrap();
                        header.originating_system = args[5].clone().str().unwrap();
                        header.originating_system = args[6].clone().str().unwrap();
                    },
                    "FILE_SCHEMA" => {
                        header.file_schema = args[0].clone().tuple().unwrap().iter().map(|v| v.clone().str().unwrap()).collect();
                    },
                    _ => {
                    }
                },
            _ => {}
        }
    }
    header
}

fn parse_data(_parsed_data: parser::Data) -> Vec<AdvancedFace> {
    Vec::new()
}

pub fn parse(s : &str) -> (Header, Vec<AdvancedFace>) {
    let parsed = parser::parse(s).unwrap();
    (parse_header(parsed.header), parse_data(parsed.data))
}
