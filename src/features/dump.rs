use crate::resource::graphic::{GraphicInfoResource, GraphicResource, PaletteResource};

pub fn dump_graphics(args: &clap::ArgMatches, resources: &mut (GraphicInfoResource, GraphicResource, Option<PaletteResource>)) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}