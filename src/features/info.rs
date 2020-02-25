use std::num;
use std::io;
use std::io::SeekFrom;
use crate::data_structure::graphic::{GraphicInfo, GraphicHeader};
use crate::resource::graphic::{GraphicInfoResource, GraphicResource};
use prettytable::{table, row, cell};

pub fn show_info<T>(args: &clap::ArgMatches, resource: &mut (GraphicInfoResource, GraphicResource, T)) -> Result<(), Box<dyn std::error::Error>>{
    let result = ArgParse::parse(args)?;
    if result.id.is_some() {
        print_table(vec![find_by_id(result.id.unwrap(),&mut resource.0,&mut resource.1)?], false);
    } else if result.all {
        print_table(find_all(&mut resource.0, &mut resource.1)?, true);
    }

    Ok(())
}

fn find_by_id(id: u32, graphic_info_resource: &mut GraphicInfoResource, graphic_resource: &mut GraphicResource) -> Result<(GraphicInfo, GraphicHeader), io::Error>{
    let graphic_info = graphic_info_resource.find(|gi| gi.id == id).unwrap();
    graphic_resource.seek(SeekFrom::Start(graphic_info.address as u64))?;
    let graphic_header = graphic_resource.read_header();

    Ok((graphic_info, graphic_header))
}

fn find_all(graphic_info_resource: &mut GraphicInfoResource, graphic_resource: &mut GraphicResource) -> Result<Vec<(GraphicInfo, GraphicHeader)>, io::Error> {
    let mut ret = vec![];

    for graphic_info in graphic_info_resource {
        graphic_resource.seek(SeekFrom::Start(graphic_info.address as u64))?;
        ret.push((graphic_info, graphic_resource.read_header()));
    }

    Ok(ret)
}

fn print_table(data: Vec<(GraphicInfo, GraphicHeader)>, skip_equal: bool) {
    let mut table = table!(["id", "GraphicInfo.bin", "Graphic.bin"]);

    for (info, header) in data {
        if info == header && !skip_equal {
            table.add_row(row![info.id, info, header]);
        } else if info != header {
            table.add_row(row![bFr => info.id, info, header]);
        }
    }

    table.printstd();
}

struct ArgParse {
    id: Option<u32>,
    all: bool
}

impl ArgParse {
    fn parse(args: &clap::ArgMatches) -> Result<Self, num::ParseIntError> {
        let id = if args.value_of("graphic_id").is_none() {
            None
        } else {
            Some(args.value_of("graphic_id").unwrap().parse::<u32>()?)
        };
        let all = args.is_present("all");

        Ok(Self{id, all})
    }
}