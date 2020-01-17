use pest::Parser;
use pest::iterators::Pairs;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "step.pest"]
struct StepParser;

struct Parsed {
    desc: String,
    name: String,
    schema: String,
    data: Vec<Vec<Data>>,
}

struct Data {
    name: String,
    args: Vec<Arg>,
}

enum Arg {
    Str(String),
    Id(u64),
    Number(f64),
    Control(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
    }
}
