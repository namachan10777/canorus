use super::preprocess;
use super::math::V3;
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

type DataDB = HashMap<u64, preprocess::Data>;

#[derive(Debug, Clone)]
pub struct Axis {
    pub p: V3,
    pub direction: V3,
    pub ref_direction: V3,
}

#[derive(Debug, Clone)]
pub enum FaceElement {
    Cylinder(f64, Axis),
    Plane(Axis),
}

#[derive(Debug, Clone)]
pub struct AdvancedFace {
    pub flag: bool,
    pub elem: FaceElement,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    HeaderParseError(String),
    DataParseError(String),
    PreprocessError(String),
}

fn parse_header(parsed_header: Vec<preprocess::Header>) -> Result<Header, ParseError> {
    let mut header = Header::default();
    for (name, args) in parsed_header {
        match name.as_str() {
            "FILE_DESCRIPTION" => {
                let e = || ParseError::HeaderParseError("FILE_DESCRIPTION".to_string());
                header.description = args[0]
                    .clone()
                    .tuple()
                    .ok_or_else(e)?
                    .iter()
                    .map(|v| v.str().cloned().ok_or_else(e))
                    .collect::<Result<Vec<String>, ParseError>>()?;
                header.implementation_level = args[1]
                    .str()
                    .ok_or_else(e)?
                    .clone();
            },
            "FILE_NAME" => {
                let e = || ParseError::HeaderParseError("FILE_NAME".to_string());
                header.name = args[0]
                    .str()
                    .ok_or_else(e)?
                    .clone();
                header.time_stamp = args[1]
                    .str()
                    .ok_or_else(e)?
                    .clone();
                header.author = args[2]
                    .clone()
                    .tuple()
                    .ok_or_else(e)?
                    .iter()
                    .map(|v| v.str().cloned().ok_or_else(e))
                    .collect::<Result<Vec<String>, ParseError>>()?;
                header.organization = args[3]
                    .clone()
                    .tuple()
                    .ok_or_else(e)?
                    .iter()
                    .map(|v| v.str().cloned().ok_or_else(e))
                    .collect::<Result<Vec<String>, ParseError>>()?;
                header.preprocessor_version = args[4]
                    .str()
                    .ok_or_else(e)?
                    .clone();
                header.originating_system = args[5]
                    .str()
                    .ok_or_else(e)?
                    .clone();
                header.authorisation = args[6]
                    .str()
                    .ok_or_else(e)?
                    .clone();
            },
            "FILE_SCHEMA" => {
                let e = || ParseError::HeaderParseError("FILE_SCHEMA".to_string());
                header.file_schema = args[0]
                    .tuple()
                    .ok_or_else(e)?
                    .iter()
                    .map(|v| v.str().cloned().ok_or_else(e))
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
    let e = || ParseError::DataParseError("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string());
    for key in map.keys() {
        if let preprocess::Data::Single(_, desc_name, _) = &map[key] {
            if let "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION" = desc_name.as_str() {
                    return Ok(*key);
            }
        }
    }
    Err(e())
}

fn get_styled_item_ids(map: &DataDB, id: u64) -> Result<Vec<u64>, ParseError> {
    let e = || ParseError::DataParseError("MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION" {
                args
                    .get(1)
                    .ok_or_else(e)?
                    .tuple()
                    .ok_or_else(e)?
                    .iter()
                    .map(|id| id.id().copied())
                    .collect::<Option<Vec<u64>>>()
                    .ok_or_else(e)
            }
            else {
                Err(e())
            }
        },
        _ => Err(e())
    }
}

fn get_manifold_solid_brep_id(map: &DataDB, id: u64) -> Result<u64, ParseError> {
    let e = || ParseError::DataParseError("STYLED_ITEM".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "STYLED_ITEM" {
                args
                    .get(2)
                    .ok_or_else(e)?
                    .id()
                    .copied()
                    .ok_or_else(e)
            }
            else {
                Err(e())
            }
        },
        _ => Err(e())
    }
}

fn get_closed_shell_id(map: &DataDB, id: u64) -> Result<u64, ParseError> {
    let e = || ParseError::DataParseError("MANIFOLD_SOLID_BREP".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "MANIFOLD_SOLID_BREP" {
                args
                    .get(1)
                    .ok_or_else(e)?
                    .id()
                    .copied()
                    .ok_or_else(e)
            }
            else {
                Err(e())
            }
        },
        _ => Err(e())
    }
}

fn get_advanced_face_ids(map: &DataDB, id: u64) -> Result<Vec<u64>, ParseError> {
    let e = || ParseError::DataParseError("CLOSED_SHELL".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "CLOSED_SHELL" {
                args
                    .get(1)
                    .ok_or_else(e)?
                    .tuple()
                    .ok_or_else(e)?
                    .iter()
                    .map(|id| id.id().copied())
                    .collect::<Option<Vec<u64>>>()
                    .ok_or_else(e)
            }
            else {
                Err(e())
            }
        },
        _ => Err(e())
    }
}
fn parse_direction(map: &DataDB, id: u64) -> Result<V3, ParseError> {
    let e = || ParseError::DataParseError("DIRECTION".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "DIRECTION" {
                let scalars =
                    args.get(1).ok_or_else(e)?
                    .tuple().ok_or_else(e)?
                    .iter().map(|x| x.float().copied()).collect::<Option<Vec<f64>>>().ok_or_else(e)?;
                if scalars.len() == 3 {
                    Ok(V3 ([
                        scalars[0],
                        scalars[1],
                        scalars[2],
                    ]))
                }
                else {
                    Err(e())
                }
            }
            else {
                Err(e())
            }
        },
        _ => Err(e())
    }
}

fn parse_cartesian_point(map: &DataDB, id: u64) -> Result<V3, ParseError> {
    let e = || ParseError::DataParseError("CARTESIAN_POINT".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "CARTESIAN_POINT" {
                let scalars =
                    args.get(1).ok_or_else(e)?
                    .tuple().ok_or_else(e)?
                    .iter().map(|x| x.float().copied()).collect::<Option<Vec<f64>>>().ok_or_else(e)?;
                if scalars.len() == 3 {
                    Ok(V3 ([
                        scalars[0],
                        scalars[1],
                        scalars[2],
                    ]))
                }
                else {
                    Err(e())
                }
            }
            else {
                Err(e())
            }
        },
        _ => Err(e())
    }
}

fn parse_ref_direction_placement_3d(map: &DataDB, id: u64) -> Result<Axis, ParseError> {
    let e = || ParseError::DataParseError("AXIS2_PLACEMENT_3D".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "AXIS2_PLACEMENT_3D" {
                let p = parse_cartesian_point(&map, *args.get(1).ok_or_else(e)?.id().ok_or_else(e)?)?;
                let direction = parse_direction(&map, *args.get(2).ok_or_else(e)?.id().ok_or_else(e)?)?;
                let ref_direction = parse_direction(&map, *args.get(3).ok_or_else(e)?.id().ok_or_else(e)?)?;
                Ok(Axis { p, direction, ref_direction })
            }
            else {
                Err(e())
            }
        },
        _ => Err(e())
    }
}

fn parse_face_element(map: &DataDB, id: u64) -> Result<FaceElement, ParseError> {
    let e = || ParseError::DataParseError("face element".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            match name.as_str() {
                "PLANE" => {
                    let axis = parse_ref_direction_placement_3d(&map, *(args.get(1).ok_or_else(e)?.id().ok_or_else(e)?))?;
                    Ok(FaceElement::Plane(axis))
                },
                "CYLINDRICAL_SURFACE" => {
                    let r = args.get(2).ok_or_else(e)?.float().ok_or_else(e)?;
                    let axis = parse_ref_direction_placement_3d(&map, *(args.get(1).ok_or_else(e)?.id().ok_or_else(e)?))?;
                    Ok(FaceElement::Cylinder(*r, axis))
                },
                _ => Err(e())
            }
        },
        _ => Err(e())
    }
}/*

type EdgeLoop = Vec<OrientedEdge>;
type OrientedEdge = Vec<EdgeCurve>;
type EdgeCurve = Vec<V3>;

enum FaceBound {
    Outer(EdgeLoop),
    Inner(EdgeLoop),
}

fn parse_face_outer(map: &DataDB, id: u64) -> FaceBound {
}

fn parse_edge_loop(map: &DataDB, id: u64) -> EdgeLoop {
}

fn parse_oriented_edge(map: &DataDB, id: u64) -> OrientedEdge {
}

fn parse_edge_curve(map: &DataDB, id: u64) -> EdgeCurve {
}

fn parse_vertex_point(map: &DataDB, id: u64) -> V3 {
}*/

fn parse_advanced_face(map: &DataDB, id: u64) -> Result<AdvancedFace, ParseError> {
    let e = || ParseError::DataParseError("ADVANCED_FACE".to_owned());
    match &map[&id] {
        preprocess::Data::Single(_, name, args) => {
            if name == "ADVANCED_FACE" {
                let flag = args.get(3).ok_or_else(e)?.boolean().ok_or_else(e)?;
                let element_id = args.get(2).ok_or_else(e)?.id().ok_or_else(e)?;
                Ok(AdvancedFace {
                    flag: *flag,
                    elem: parse_face_element(&map, *element_id)?
                })
            }
            else {
                Err(e())
            }
        }
        _ => Err(e())
    }
}

fn parse_data(parsed_data: Vec<preprocess::Data>) -> Result<Vec<AdvancedFace>, ParseError> {
    let map = make_db(parsed_data);
    // ad-hoc
    let root_id = find_mechanical_design_geometric_presentation_representation_id(&map)?;
    let styled_item_ids = get_styled_item_ids(&map, root_id)?;
    let manifold_solid_brep_id = get_manifold_solid_brep_id(&map, styled_item_ids[0])?;
    let closed_shell_id = get_closed_shell_id(&map, manifold_solid_brep_id)?;
    let advanced_face_ids = get_advanced_face_ids(&map, closed_shell_id)?;
    advanced_face_ids.iter().map(|face_id| parse_advanced_face(&map, *face_id)).collect()
}

pub fn parse(s : &str) -> Result<(Header, Vec<AdvancedFace>), ParseError> {
    let parsed = preprocess::parse(s).map_err(
        |e| match e {
            preprocess::PreprocessError::Fail(info) => {
                match info {
                    preprocess::PreprocessErrorInfo::LineCol((l, c)) =>
                        ParseError::PreprocessError(format!("irregular syntax at {}:{}", l, c)),
                    preprocess::PreprocessErrorInfo::Span((l1, c1), (l2, c2)) =>
                        ParseError::PreprocessError(format!("irregular syntax in {}:{} -- {}:{}", l1, c1, l2, c2))
                }
            },
            preprocess::PreprocessError::InternalError => {
                ParseError::PreprocessError("internal parser error".to_owned())
            }
        })?;
    Ok((parse_header(parsed.header)?, parse_data(parsed.data)?))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::Read;
    use super::preprocess;

    fn prepare_test_data() -> preprocess::Step {
        let mut f = fs::File::open("./example.STEP").unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        preprocess::parse(&buf).unwrap()
    }

    #[test]
    fn test_parse_header() {
        let header = parse_header(prepare_test_data().header).unwrap();
        assert_eq!(
            header.description,
            vec![""]);
        assert_eq!(
            header.implementation_level,
            "2;1");
        assert_eq!(
            header.name,
            r"D:\\Desktop\\Part1.stp");
        assert_eq!(
            header.time_stamp,
            "2020-01-17T21:29:41+09:00");
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

    #[test]
    fn test_find_mechanical_design_geometric_presentation_representation_id() {
        let data = make_db(prepare_test_data().data);
        assert_eq!(find_mechanical_design_geometric_presentation_representation_id(&data),
            Ok(10));
    }

    #[test]
    fn test_parse_data() {
        let data = prepare_test_data();
        parse_data(data.data).unwrap();
    }
}
