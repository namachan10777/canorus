use nom:: {
    bytes::complete::{escaped, take_while},
    character::complete::{alphanumeric1 as alphanumeric, one_of, char, digit1},
    combinator::{cut, map},
    error::{ParseError, ErrorKind, context},
    sequence::{preceded, terminated},
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

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alphanumeric, '\\', one_of("\"n\\"))(i)
}

fn string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    context("string", preceded(char('\''), cut(terminated(parse_str, char('\'')))))(i)
}

fn id <'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, u64, E> {
    context("id", preceded(char('#'), map(digit1, |s: &'a str| { let i: u64 = s.parse().unwrap(); i })))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(string::<(&str, ErrorKind)>("'hoge\\n'"), Ok(("", "hoge\\n")));
        assert_eq!(id::<(&str, ErrorKind)>("#112"), Ok(("", 112)));
    }
}
