use pest::Parser;
use pest::iterators::{Pair};
use pest::error::Error;
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
    Control(String),
    Tuple(Vec<Value>),
    Wildcard,
    Dollar,
    Desc(String, Vec<Value>),
}

impl Value {
    pub fn str(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn tuple(self) -> Option<Vec<Value>> {
        match self {
            Value::Tuple(s) => Some(s),
            _ => None,
        }
    }
}

pub type Header = Vec<Value>;
pub type Data = Vec<(u64, Value)>;

#[derive(Debug)]
pub struct Step {
    pub header: Header,
    pub data: Data,
}

fn value(v: Pair<Rule>) -> Value {
    match v.as_rule() {
        Rule::value => {
            let v = v.into_inner();
            match v.peek().unwrap().as_rule() {
                Rule::float => Value::Float(v.as_str().parse().unwrap()),
                Rule::integer => Value::Int(v.as_str().parse().unwrap()),
                Rule::string => {
                    let s = v.as_str();
                    Value::String(s[1..s.len()-1].to_string())
                },
                Rule::id => Value::Id(v.as_str()[1..].parse().unwrap()),
                Rule::control => Value::Control(v.as_str()[1..].to_string()),
                Rule::tuple => {
                    let inner = v.peek().unwrap().into_inner().map(|v| value(v)).collect();
                    Value::Tuple(inner)
                },
                Rule::wildcard => Value::Wildcard,
                Rule::dollar => Value::Wildcard,
                Rule::desc => {
                    let mut inner = v.peek().unwrap().into_inner();
                    let name = inner.next().unwrap().as_str();
                    Value::Desc(
                        name.to_string(),
                        inner.map(|v| value(v)).collect(),
                    )
                },
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}

fn elem(e: Pair<Rule>) -> (u64, Value) {
    match e.as_rule() {
        Rule::elem => {
            let mut inner = e.into_inner();
            let id = inner.next().unwrap().as_str()[1..].parse().unwrap();
            let body = value(inner.next().unwrap());
            (id, body)
        },
        _ => unreachable!(),
    }
}

fn header(h: Pair<Rule>) -> Header {
    match h.as_rule() {
        Rule::header => {
            let inner = h.into_inner();
            inner.map(|v| value(v)).collect()
        },
        _ => unreachable!(),
    }
}

fn data(d: Pair<Rule>) -> Data {
    match d.as_rule() {
        Rule::data => {
            let inner = d.into_inner();
            inner.map(|v| elem(v)).collect()
        },
        _ => unreachable!(),
    }
}

fn step(s: Pair<Rule>) -> Step {
    match s.as_rule() {
        Rule::step => {
            let mut inner = s.into_inner();
            let header = header(inner.next().unwrap());
            let data = data(inner.next().unwrap());
            Step {
                header,
                data,
            }
        },
        _ => unreachable!()
    }
}

pub fn parse<'a>(input: &'a str) -> Result<Step, Error<Rule>> {
    StepParser::parse(Rule::step, input).map(|v| step(v.peek().unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(StepParser::parse(Rule::value, "-1.1E-15").map(|v| value(v.peek().unwrap())), Ok(Value::Float(-1.1e-15)));
        assert_eq!(StepParser::parse(Rule::value, "10.").map(|v| value(v.peek().unwrap())), Ok(Value::Float(10.)));
        assert_eq!(StepParser::parse(Rule::value, "#123").map(|v| value(v.peek().unwrap())), Ok(Value::Id(123)));
        assert_eq!(StepParser::parse(Rule::value, ".BAR").map(|v| value(v.peek().unwrap())), Ok(Value::Control("BAR".to_string())));
        assert_eq!(StepParser::parse(Rule::value, "(1., '', #12)").map(|v| value(v.peek().unwrap())),
            Ok(Value::Tuple(vec![Value::Float(1.0), Value::String(String::new()), Value::Id(12)])));
        assert_eq!(StepParser::parse(Rule::value, "('')").map(|v| value(v.peek().unwrap())), 
            Ok(Value::Tuple(vec![Value::String(String::new())])));
        assert_eq!(StepParser::parse(Rule::elem,
            "#20=FACE_BOUND('',#64,.T.);"
            ).map(|v| elem(v.peek().unwrap())),
            Ok((20,
                Value::Desc(
                "FACE_BOUND".to_string(),
                vec![
                    Value::String("".to_string()),
                    Value::Id(64),
                    Value::Control("T.".to_string()),
                ]))));
    }
}
