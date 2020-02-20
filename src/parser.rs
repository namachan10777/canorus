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
    authorisation: String,
    file_schema: Vec<String>,
}

type V3 = (f64, f64, f64);
type DataDB = HashMap<u64, preprocess::Data>;

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
    DataParseError(String),
}

fn parse_header(parsed_header: Vec<preprocess::Header>) -> Result<Header, ParseError> {
    let mut header = Header::default();
    for (name, args) in parsed_header {
        match name.as_str() {
            "FILE_DESCRIPTION" => {
                header.description = args[0]
                    .clone()
                    .tuple()
                    .ok_or(ParseError::HeaderParseError("FILE_DESCRIPTION".to_string()))?
                    .iter()
                    .map(|v| v
                        .str()
                        .map(|s| s.clone())
                        .ok_or(ParseError::HeaderParseError("FILE_DESCRIPTION".to_string())))
                    .collect::<Result<Vec<String>, ParseError>>()?;
                header.implementation_level = args[1]
                    .str()
                    .ok_or(ParseError::HeaderParseError("FILE_DESCRIPTION".to_string()))?
                    .clone();
            },
            "FILE_NAME" => {
                header.name = args[0]
                    .str()
                    .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                    .clone();
                header.time_stamp = args[1]
                    .str()
                    .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                    .clone();
                header.author = args[2]
                    .clone()
                    .tuple()
                    .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                    .iter()
                    .map(|v| v.str().map(|s| s.clone()).ok_or(ParseError::HeaderParseError("FILE_NAME".to_string())))
                    .collect::<Result<Vec<String>, ParseError>>()?;
                header.organization = args[3]
                    .clone()
                    .tuple()
                    .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                    .iter()
                    .map(|v| v.str().map(|s| s.clone()).ok_or(ParseError::HeaderParseError("FILE_NAME".to_string())))
                    .collect::<Result<Vec<String>, ParseError>>()?;
                header.preprocessor_version = args[4]
                    .str()
                    .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                    .clone();
                header.originating_system = args[5]
                    .str()
                    .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                    .clone();
                header.authorisation = args[6]
                    .str()
                    .ok_or(ParseError::HeaderParseError("FILE_NAME".to_string()))?
                    .clone();
            },
            "FILE_SCHEMA" => {
                header.file_schema = args[0]
                    .tuple()
                    .ok_or(ParseError::HeaderParseError("FILE_SCHEMA".to_string()))?
                    .iter()
                    .map(|v| v.str().map(|s| s.clone()).ok_or(ParseError::HeaderParseError("FILE_SCHEMA".to_string())))
                    .collect::<Result<Vec<String>, ParseError>>()?
                    .clone();
            },
            _ => {
            }
        }
    }
    Ok(header)
}

fn make_db(data: Vec<preprocess::Data>) -> HashMap<u64, preprocess::Data> {
    let mut map = HashMap::new();
    for d in data {
        match d {
            preprocess::Data::Single(id, _, _) => {
                map.insert(id, d);
            },
            preprocess::Data::Aggregate(id, _) => {
                map.insert(id, d);
            },
        }
    }
    map
}

fn find_mechanical_design_geometric_presentation_representation_id(map: &DataDB) -> Result<u64, ParseError> {
    for key in map.keys() {
        match &map[key] {
            preprocess::Data::Single(_, desc_name, args) => {
                match desc_name.as_str() {
                    "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION" => {
                        let styled_item_ids = args
                            .get(1)
                            .ok_or(ParseError::DataParseError("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string()))?
                            .tuple()
                            .ok_or(ParseError::DataParseError("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string()))?;
                        return styled_item_ids
                            .get(1)
                            .ok_or(ParseError::DataParseError("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string()))?
                            .id()
                            .map(|id| *id)
                            .ok_or(ParseError::DataParseError("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string()));
                    },
                    _ => {
                    },
                }
            },
            _ => {},
        }
    }
    Err(ParseError::DataParseError("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string())) 
}

fn get_styled_item_ids(map: &DataDB, id: u64) -> Option<Vec<u64>> {
    None
}

fn get_manifold_solid_brep_id(map: &DataDB, id: u64) -> Option<u64> {
    None
}

fn get_closed_shell_id(map: &DataDB, id: u64) -> Option<u64> {
    None
}

fn get_advanced_face_ids(map: &DataDB, id: u64) -> Option<Vec<u64>> {
    None
}

fn parse_advanced_face(map: &DataDB, id: u64) -> Option<AdvancedFace> {
    None
}

fn parse_data(parsed_data: Vec<preprocess::Data>) -> Vec<AdvancedFace> {
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::Read;
    use super::preprocess;

    #[test]
    fn test_parse_header() {
        let mut f = fs::File::open("./example.STEP").unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        let step = preprocess::parse(&buf).unwrap();
        let header = parse_header(step.header).unwrap();

        assert_eq!(
            header.description,
            vec![""]);
        assert_eq!(
            header.implementation_level,
            "2;1");
        assert_eq!(
            header.name,
            r"D:\\Desktop\\\X2\8DB3\X0\-\X2\30AD30E330B930BF7528\X0\.stp");
        assert_eq!(
            header.time_stamp,
            "2019-09-29T23:06:55+09:00");
        assert_eq!(
            header.author,
            vec!["Tsuba"]);
        assert_eq!(
            header.organization,
            vec![""]);
        assert_eq!(
            header.preprocessor_version,
            "ST-DEVELOPER v17");
        assert_eq!(
            header.originating_system,
            "Autodesk Inventor 2018");
        assert_eq!(
            &header.authorisation,
            "");
        assert_eq!(
            header.file_schema,
            vec!["AUTOMOTIVE_DESIGN { 1 0 10303 214 3 1 1 }"]);
    }
}
