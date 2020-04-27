use crate::data_structure::graphic::{GraphicInfo, GraphicHeader, GraphicV1};
use crate::features::ArgParse;
use crate::resource::graphic::{GraphicInfoResource, GraphicResource, PaletteResource};
use log::{debug, info, trace};
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
        let (graphic_info, mut graphic) =
            find_by_id(result.id.unwrap(), &mut resources.0, &mut resources.1)?;
        let palette = resources.2.as_mut().unwrap().build()?;
        debug!("{:?}", *graphic);
        debug!("{:?}", palette);

        info!("Decoding graphic data");
        if (*graphic).header.version & 1 == 1 {
            (*graphic).data = (*graphic).data.decode();
        }
        info!("Decoded graphic data");

        info!("Building image");
        if let Some(image) = (*graphic).build_image(&graphic_info, &palette)? {
            image.save(
                format!(
                    "{}/{}.bmp",
                    result.output.unwrap(),
                    graphic_info.id
                )
            )?;
        }
        info!("Built image");
    } else if result.all {
        let palette = resources.2.as_mut().unwrap().build()?;

        for (info, mut graphic) in find_all(&mut resources.0, &mut resources.1)? {
            trace!("Graphic ID: {}", info.id);
            if (*graphic).header.version & 1 == 1 {
                (*graphic).data = (*graphic).data.decode();
            }
            if let Some(image) = (*graphic).build_image(&info, &palette)? {
                image.save(
                    format!(
                        "{}/{}.bmp",
                        result.output.unwrap(),
                        info.id
                    )
                )?;
            }
        }
    }

    Ok(())
}

fn get_graphic(info: &GraphicInfo, header: &GraphicHeader, graphic: &mut GraphicResource) -> Result<Box<GraphicV1>, Box<dyn std::error::Error>> {
    let graphic = match header.version {
        0 | 1 => {
            graphic.seek(SeekFrom::Start(info.address as u64))?;
            Box::new(GraphicV1::new(
                graphic.read(info.length as usize)?,
            )?)
        },
        2 | 3 => {
            todo!()
        }
        _ => {
            panic!("Unknown version of graphic.");
        }
    };

    Ok(graphic)
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

    let graphic = get_graphic(&graphic_info, &graphic_header, graphic_resource)?;

    Ok((graphic_info, graphic))
}

fn find_all(
    graphic_info_resource: &mut GraphicInfoResource,
    graphic_resource: &mut GraphicResource,
) -> Result<Vec<(GraphicInfo, Box<GraphicV1>)>, Box<dyn std::error::Error>> {
    let mut ret = vec![];

    info!("Collecting all of GraphicInfo and GraphicHeader");
    for graphic_info in graphic_info_resource {
        graphic_resource.seek(SeekFrom::Start(graphic_info.address as u64))?;
        let header = graphic_resource.read_header()?;
        // 圖片版本 > 2 的功能尚未完成，先行跳過
        if header.version <= 1 {
            let graphic = get_graphic(&graphic_info, &header, graphic_resource)?;
            ret.push((graphic_info, graphic));
        }
    }
    info!("Collected all of GraphicInfo and GraphicHeader");

    Ok(ret)
}
