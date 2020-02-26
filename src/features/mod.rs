pub mod info;
pub mod dump;

#[derive(Debug)]
pub struct ArgParse<'a> {
    id: Option<u32>,
    all: bool,
    output: Option<&'a str>,
}

impl<'a> ArgParse<'a> {
    pub fn parse(args: &'a clap::ArgMatches) -> Result<Self, std::num::ParseIntError> {
        let id = if args.value_of("graphic_id").is_none() {
            None
        } else {
            Some(args.value_of("graphic_id").unwrap().parse::<u32>()?)
        };
        let all = args.is_present("all");
        let output = if args.value_of("output").is_none() {
            None
        } else {
            Some(args.value_of("output").unwrap())
        };

        Ok(Self{id, all, output})
    }
}