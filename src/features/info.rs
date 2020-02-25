use std::num;
use std::io::SeekFrom;
use crate::resource::graphic::{GraphicInfoResource, GraphicResource};

pub fn show_info<T>(args: &clap::ArgMatches, resource: &mut (GraphicInfoResource, GraphicResource, T)) -> Result<(), Box<dyn std::error::Error>>{
    let result = ArgParse::parse(args)?;
    let graphic_info = resource.0.find(|gi| gi.id == result.id).unwrap();
    resource.1.seek(SeekFrom::Start(graphic_info.address as u64))?;
    let graphic_header = resource.1.read_header();

    println!("{:?}", graphic_info);
    println!("{:?}", graphic_header);

    Ok(())
}

struct ArgParse {
    id: u32,
}

impl ArgParse {
    fn parse(args: &clap::ArgMatches) -> Result<Self, num::ParseIntError> {
        if args.value_of("graphic_id").is_none() {
            Ok(Self {id: 0})
        } else {
            Ok(Self {
                id: args.value_of("graphic_id").unwrap().parse::<u32>()?
            })
        }
    }
}