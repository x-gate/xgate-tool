pub mod info;
pub mod dump;

#[derive(Debug)]
pub struct ArgParse {
    id: Option<u32>,
    all: bool
}

impl ArgParse {
    pub fn parse(args: &clap::ArgMatches) -> Result<Self, std::num::ParseIntError> {
        let id = if args.value_of("graphic_id").is_none() {
            None
        } else {
            Some(args.value_of("graphic_id").unwrap().parse::<u32>()?)
        };
        let all = args.is_present("all");

        Ok(Self{id, all})
    }
}