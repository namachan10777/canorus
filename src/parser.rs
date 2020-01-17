use nom;

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
