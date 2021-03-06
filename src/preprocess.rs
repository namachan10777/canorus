use pest::Parser;
use pest::iterators::{Pair, Pairs};
use std::vec::Vec;

#[derive(Parser)]
#[grammar = "step.pest"]
struct StepParser;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i64),
    String(String),
    Id(u64),
    Bool(bool),
    Enum(String),
    Tuple(Vec<Value>),
    Xplicit,
    Undefined,
    Desc(String, Vec<Value>),
}

#[derive(Debug, PartialEq)]
pub enum PreprocessErrorInfo {
    LineCol((usize, usize)),
    Span((usize, usize), (usize, usize)),
}

#[derive(Debug, PartialEq)]
pub enum PreprocessError {
    Fail(PreprocessErrorInfo),
    InternalError,
}

impl From<std::num::ParseFloatError> for PreprocessError {
    fn from(_: std::num::ParseFloatError) -> Self {
        PreprocessError::InternalError
    }
}

impl From<std::num::ParseIntError> for PreprocessError {
    fn from(_: std::num::ParseIntError) -> Self {
        PreprocessError::InternalError
    }
}

impl From<pest::error::Error<Rule>> for PreprocessError {
    fn from (e: pest::error::Error<Rule>) -> Self {
        match e.line_col {
            pest::error::LineColLocation::Pos(p) => PreprocessError::Fail(PreprocessErrorInfo::LineCol(p)),
            pest::error::LineColLocation::Span(p1, p2) => PreprocessError::Fail(PreprocessErrorInfo::Span(p1, p2)),
        }
    }
}

impl Value {
    pub fn str(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn tuple(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Tuple(s) => Some(s),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&u64> {
        match self {
            Value::Id(id) => Some(id),
            _ => None,
        }
    }

    pub fn boolean(&self) -> Option<&bool> {
        match self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<&f64> {
        match self {
            Value::Float(f) => Some(f),
            _ => None,
        }
    }
}

pub type Header = (String, Vec<Value>);

#[derive(Debug, PartialEq)]
pub enum Data {
    Single(u64, String, Vec<Value>),
    Aggregate(u64, Vec<(String, Vec<Value>)>),
}

#[derive(Debug)]
pub struct Step {
    pub header: Vec<Header>,
    pub data: Vec<Data>,
}

fn value(v: Pair<Rule>) -> Result<Value, PreprocessError> {
    match v.as_rule() {
        Rule::value => {
            let v = v.into_inner();
            match v.peek().ok_or(PreprocessError::InternalError)?.as_rule() {
                Rule::float => {
                    let f: Result<f64, PreprocessError> = v.as_str().parse().map_err(From::from);
                    Ok(Value::Float(f?))
                },
                Rule::integer => {
                    let i: Result<i64, PreprocessError> = v.as_str().parse().map_err(From::from);
                    Ok(Value::Int(i?))
                },
                Rule::string => {
                    let s = v.as_str();
                    Ok(Value::String(s.get(1..s.len()-1).ok_or(PreprocessError::InternalError)?.to_string()))
                },
                Rule::id => {
                    let id: Result<u64, PreprocessError> = v
                        .as_str()
                        .get(1..)
                        .ok_or(PreprocessError::InternalError)?
                        .parse()
                        .map_err(From::from);
                    Ok(Value::Id(id?))
                },
                Rule::enum_ => {
                    let s = v.as_str();
                    Ok(Value::Enum(s.get(1..s.len()-1).ok_or(PreprocessError::InternalError)?.to_string()))
                },
                Rule::bool_ => {
                    Ok(Value::Bool(v.as_str() == ".T."))
                },
                Rule::tuple => {
                    let inner : Result<Vec<Value>, PreprocessError> = v
                        .peek()
                        .ok_or(PreprocessError::InternalError)?
                        .into_inner()
                        .map(value).collect();
                    Ok(Value::Tuple(inner?))
                },
                Rule::xplicit => Ok(Value::Xplicit),
                Rule::undefined => Ok(Value::Undefined),
                Rule::desc => {
                    let (name, args) =
                        desc(v
                        .peek()
                        .ok_or(PreprocessError::InternalError)?)?;
                    Ok(Value::Desc(name, args))
                },
                _ => Err(PreprocessError::InternalError),
            }
        },
        _ => Err(PreprocessError::InternalError),
    }
}

fn desc(d: Pair<Rule>) -> Result<(String, Vec<Value>), PreprocessError> {
    match d.as_rule() {
        Rule::desc => {
            let mut inner = d.into_inner();
            let name = inner
                .next()
                .ok_or(PreprocessError::InternalError)?
                .as_str();
            let args: Result<Vec<Value>,PreprocessError> = inner
                .map(value)
                .collect();
            Ok((
                name.to_string(),
                args?,
            ))
        },
        _ => Err(PreprocessError::InternalError),
    }
}

fn header(h: Pair<Rule>) -> Result<Vec<Header>, PreprocessError> {
    match h.as_rule() {
        Rule::header => {
            let inner = h.into_inner();
            inner.map(desc).collect()
        },
        _ => Err(PreprocessError::InternalError),
    }
}

fn data(d: Pair<Rule>) -> Result<Vec<Data>, PreprocessError> {
    match d.as_rule() {
        Rule::data => Ok(d
            .into_inner()
            .map(|d| {
                let mut inner = d.into_inner();
                let id : Result<u64, PreprocessError> = inner
                    .next()
                    .ok_or(PreprocessError::InternalError)?
                    .as_str()
                    .get(1..)
                    .ok_or(PreprocessError::InternalError)?
                    .parse::<u64>()
                    .map_err(From::from);
                let id = id?;
                let right = inner
                    .next()
                    .ok_or(PreprocessError::InternalError)?;
                match right.as_rule() {
                    Rule::desc =>
                        desc(right)
                        .map(|(name, args)| Data::Single(id, name, args)),
                    Rule::aggregate =>
                        right
                        .into_inner()
                        .map(desc)
                        .collect::<Result<Vec<(String, Vec<Value>)>, PreprocessError>>()
                        .map(|aggregated| Data::Aggregate(id, aggregated)),
                    _ => Err(PreprocessError::InternalError)
                }
            })
            .collect::<Result<Vec<Data>, PreprocessError>>()?),
        _ => Err(PreprocessError::InternalError),
    }
}

fn step(s: Pair<Rule>) -> Result<Step, PreprocessError> {
    match s.as_rule() {
        Rule::step => {
            let mut inner = s.into_inner();
            let header = header(inner.next().ok_or(PreprocessError::InternalError)?)?;
            let data = data(inner.next().ok_or(PreprocessError::InternalError)?)?;
            Ok(Step {
                header,
                data,
            })
        },
        _ => Err(PreprocessError::InternalError)
    }
}

pub fn parse(input: &str) -> Result<Step, PreprocessError> {
    let parsed: Result<Pairs<Rule>, PreprocessError> = StepParser::parse(Rule::step, input)
        .map_err(From::from);
    step(parsed?.peek().ok_or(PreprocessError::InternalError)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(StepParser::parse(Rule::value, "-1.1E-15").map(|v| value(v.peek().unwrap()).unwrap()), Ok(Value::Float(-1.1e-15)));
        assert_eq!(StepParser::parse(Rule::value, "10.").map(|v| value(v.peek().unwrap()).unwrap()), Ok(Value::Float(10.)));
        assert_eq!(StepParser::parse(Rule::value, "#123").map(|v| value(v.peek().unwrap()).unwrap()), Ok(Value::Id(123)));
        assert_eq!(StepParser::parse(Rule::value, ".BAR.").map(|v| value(v.peek().unwrap()).unwrap()), Ok(Value::Enum("BAR".to_string())));
        assert_eq!(StepParser::parse(Rule::value, "(1., '', #12)").map(|v| value(v.peek().unwrap()).unwrap()),
            Ok(Value::Tuple(vec![Value::Float(1.0), Value::String(String::new()), Value::Id(12)])));
        assert_eq!(StepParser::parse(Rule::value, "('')").map(|v| value(v.peek().unwrap()).unwrap()), 
            Ok(Value::Tuple(vec![Value::String(String::new())])));
        assert_eq!(StepParser::parse(Rule::data,
            r"DATA;
            #10=MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION('',(#13),#501);
            ENDSEC;"
            ).map(|v| data(v.peek().unwrap()).unwrap()),
            Ok(vec![
                Data::Single(10,
                "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION".to_string(),
                vec![
                    Value::String("".to_string()),
                    Value::Tuple(vec![
                        Value::Id(13),
                    ]),
                    Value::Id(501),
                ])
            ]));
    }
}

