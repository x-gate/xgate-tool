use crate::data_structure::graphic::{GraphicInfo, GraphicV1};
use crate::features::ArgParse;
use crate::resource::graphic::{GraphicInfoResource, GraphicResource, PaletteResource};
use log::{debug, info};
use std::io::SeekFrom;

pub fn dump_graphics(
    args: &clap::ArgMatches,
    resources: &mut (
        GraphicInfoResource,
        GraphicResource,
        Option<PaletteResource>,
    ),
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Parsing sub command arguments");
    let result = ArgParse::parse(args)?;
    info!("Parsed sub command arguments");
    debug!("{:?}", result);

    if result.id.is_some() {
        let (graphic_info, mut graphic) = find_by_id(result.id.unwrap(), &mut resources.0, &mut resources.1)?;
        let palette = resources.2.as_mut().unwrap().build()?;
        debug!("{:?}", *graphic);
        debug!("{:?}", palette);

        info!("Decoding graphic data");
        if (*graphic).header.version & 1 == 1 {
            (*graphic).data = (*graphic).data.decode();
        }
        info!("Decoded graphic data");

        info!("Building image");
        (*graphic).build_image(&graphic_info, &palette)?
            .save(format!("{}/{}.bmp", result.output.unwrap(), graphic_info.id))?;
        info!("Built image");
    } else if result.all {
    }

    Ok(())
}

fn find_by_id(
    id: u32,
    graphic_info_resource: &mut GraphicInfoResource,
    graphic_resource: &mut GraphicResource,
) -> Result<(GraphicInfo, Box<GraphicV1>), Box<dyn std::error::Error>> {
    info!("Finding graphic by id = {}", id);
    let graphic_info = graphic_info_resource.find(|gi| gi.id == id).unwrap();
    debug!("Found graphic_info = {:?}", graphic_info);
    info!("Finding graphic_header at {}", graphic_info.address);
    graphic_resource.seek(SeekFrom::Start(graphic_info.address as u64))?;
    let graphic_header = graphic_resource.read_header()?;
    debug!("Found graphic_header = {:?}", graphic_header);

    let graphic = match graphic_header.version {
        0 | 1 => {
            graphic_resource.seek(SeekFrom::Start(graphic_info.address as u64))?;
            Box::new(GraphicV1::new(graphic_resource.read(graphic_info.length as usize)?)?)
        }
        2 | 3 => {
            todo!();
        }
        _ => {
            panic!("Unknown version of graphic.");
        }
    };

    Ok((graphic_info, graphic))
}
