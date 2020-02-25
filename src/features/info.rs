use std::num;
use crate::resource::graphic::{GraphicInfoResource, GraphicResource};

pub fn show_info<T>(args: &clap::ArgMatches, resource: (GraphicInfoResource, GraphicResource, T)) {
    let result = ArgParse::parse(args);
}

struct ArgParse {
    id: u32,
}

impl ArgParse {
    fn parse(args: &clap::ArgMatches) -> Result<Self, num::ParseIntError> {
        if args.value_of("id").is_none() {
            Ok(Self {id: 0})
        } else {
            Ok(Self {
                id: args.value_of("id").unwrap().parse::<u32>()?
            })
        }
    }
}