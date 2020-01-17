use nom:: {
    bytes::complete::{take_while},
    error::{ParseError},
    IResult,
};

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

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}
