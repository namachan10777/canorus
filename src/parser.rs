use super::preprocess;
use std::collections::HashMap;

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

#[derive(Debug)]
pub enum ParseError {
    HeaderParseError(String),
}

fn parse_header(parsed_header: preprocess::Header) -> Result<Header, ParseError> {
    let mut header = Header::default();
    for desc in parsed_header {
        match desc {
            preprocess::Value::Desc(name, args) =>
                match name.as_str() {
                    "FILE_DESCRIPTION" => {
                        header.description = args[0]
                            .clone()
                            .tuple()
                            .ok_or(ParseError::HeaderParseError("FILE_DESCRIPTION".to_string()))?
                            .iter()
                            .map(|v| v
                                .clone()
                                .str()
                                .ok_or(ParseError::HeaderParseError("FILE_DESCRIPTION".to_string())))
                            .collect::<Result<Vec<String>, ParseError>>()?;
                        header.implementation_level = args[1]
                            .clone()
                            .str()
                            .ok_or(ParseError::HeaderParseError("FILE_DESCRIPTION".to_string()))?;
                    },
                    "FILE_NAME" => {
                        header.name = args[0]
                            .clone()
                            .str()
                            .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?;
                        header.time_stamp = args[1]
                            .clone()
                            .str()
                            .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?;
                        header.author = args[2]
                            .clone()
                            .tuple()
                            .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                            .iter()
                            .map(|v| v.clone().str().ok_or(ParseError::HeaderParseError("FILE_NAME".to_string())))
                            .collect::<Result<Vec<String>, ParseError>>()?;
                        header.organization = args[3]
                            .clone()
                            .tuple()
                            .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                            .iter()
                            .map(|v| v.clone().str().ok_or(ParseError::HeaderParseError("FILE_NAME".to_string())))
                            .collect::<Result<Vec<String>, ParseError>>()?;
                        header.preprocessor_version = args[4]
                            .clone()
                            .str()
                            .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?;
                        header.originating_system = args[5]
                            .clone()
                            .str()
                            .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?;
                        header.originating_system = args[6]
                            .clone()
                            .str()
                            .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?;
                    },
                    "FILE_SCHEMA" => {
                        header.file_schema = args[0]
                            .clone()
                            .tuple()
                            .ok_or(ParseError::HeaderParseError("FILE_SCHEMA".to_string()))?
                            .iter()
                            .map(|v| v.clone().str().ok_or(ParseError::HeaderParseError("FILE_SCHEMA".to_string())))
                            .collect::<Result<Vec<String>, ParseError>>()?;
                    },
                    _ => {
                    }
                },
            _ => {}
        }
    }
    Ok(header)
}

fn make_db(data: preprocess::Data) -> HashMap<u64, preprocess::Value> {
    let mut map = HashMap::new();
    for (id, desc) in data {
        map.insert(id, desc);
    }
    map
}

fn find_mechanical_design_geometric_presentation_representation_id(map: &HashMap<u64,preprocess::Value>) -> Option<u64> {
    for key in map.keys() {
        match &map[key] {
            preprocess::Value::Desc(desc_name, args) => {
                match desc_name.as_str() {
                    "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION" => {
                    },
                    _ => {
                    },
                }
            },
            _ => {},
        }
    }
    None
}

fn get_styled_item_ids(map: &HashMap<u64,preprocess::Value>, id: u64) -> Option<Vec<u64>> {
    None
}

fn get_manifold_solid_brep_id(map: &HashMap<u64,preprocess::Value>, id: u64) -> Option<u64> {
    None
}

fn get_closed_shell_id(map: &HashMap<u64,preprocess::Value>, id: u64) -> Option<u64> {
    None
}

fn get_advanced_face_ids(map: &HashMap<u64,preprocess::Value>, id: u64) -> Option<Vec<u64>> {
    None
}

fn parse_advanced_face(map: &HashMap<u64, preprocess::Value>, id: u64) -> Option<AdvancedFace> {
    None
}

fn parse_data(parsed_data: preprocess::Data) -> Vec<AdvancedFace> {
    let map = make_db(parsed_data);
    // ad-hoc
    let root_id = find_mechanical_design_geometric_presentation_representation_id(&map).unwrap();
    let styled_item_ids = get_styled_item_ids(&map, root_id).unwrap();
    let manifold_solid_brep_id = get_manifold_solid_brep_id(&map, styled_item_ids[0]).unwrap();
    let closed_shell_id = get_closed_shell_id(&map, manifold_solid_brep_id).unwrap();
    let advanced_face_ids = get_advanced_face_ids(&map, closed_shell_id).unwrap();
    advanced_face_ids.iter().map(|face_id| parse_advanced_face(&map, *face_id).unwrap()).collect()
}

pub fn parse(s : &str) -> Result<(Header, Vec<AdvancedFace>), ParseError> {
    let parsed = preprocess::parse(s).unwrap();
    Ok((parse_header(parsed.header)?, parse_data(parsed.data)))
}
