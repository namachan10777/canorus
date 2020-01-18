use pest::Parser;
use pest::iterators::{Pairs, Pair};
use pest::error::Error;
use std::vec::Vec;

#[derive(Parser)]
#[grammar = "step.pest"]
struct StepParser;

#[derive(PartialEq, Debug)]
enum Value {
    Number(f64),
    String(String),
    Id(u64),
    Control(String),
    Tuple(Vec<Value>),
}

#[derive(PartialEq, Debug)]
struct Desc {
    name: String,
    value: Value,
}

fn value(v: Pair<Rule>) -> Value {
    match v.as_rule() {
        Rule::value => {
            let v = v.into_inner();
            match v.peek().unwrap().as_rule() {
                Rule::float => Value::Number(v.as_str().parse().unwrap()),
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
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}

fn desc(d: Pair<Rule>) -> Desc {
    match d.as_rule() {
        Rule::desc => {
            println!("{:?}", d);
            let mut inner = d.into_inner();
            let name = inner.next().unwrap().as_str();
            let value = value(inner.next().unwrap());
            Desc { name: name.to_string(), value }
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(StepParser::parse(Rule::value, "-1.1E-15").map(|v| value(v.peek().unwrap())), Ok(Value::Number(-1.1e-15)));
        assert_eq!(StepParser::parse(Rule::value, "'hoge\\'foo'").map(|v| value(v.peek().unwrap())), Ok(Value::String("hoge\\'foo".to_string())));
        assert_eq!(StepParser::parse(Rule::value, "#123").map(|v| value(v.peek().unwrap())), Ok(Value::Id(123)));
        assert_eq!(StepParser::parse(Rule::value, ".BAR").map(|v| value(v.peek().unwrap())), Ok(Value::Control("BAR".to_string())));
        assert_eq!(StepParser::parse(Rule::value, "(1., '', #12)").map(|v| value(v.peek().unwrap())),
            Ok(Value::Tuple(vec![Value::Number(1.0), Value::String(String::new()), Value::Id(12)])));
        assert_eq!(StepParser::parse(Rule::value, "('')").map(|v| value(v.peek().unwrap())), 
            Ok(Value::Tuple(vec![Value::String(String::new())])));
        assert_eq!(StepParser::parse(Rule::desc,
                "FILE_DESCRIPTION(\
                /* description */ (''),\
                /* implementation_level */ '2;1');"
                ).map(|v| desc(v.peek().unwrap())), 
            Ok(Desc {
                name: "FILE_DESCRIPTION".to_string(),
                value: Value::Tuple(vec![
                    Value::Tuple(vec![Value::String(String::new())]),
                    Value::String("2;1".to_string()),
                ])
            }));
    }
}
