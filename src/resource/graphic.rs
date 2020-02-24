use std::io;
use std::fs::File;

pub struct GraphicInfoResource(File);
pub struct GraphicResource(File);
pub struct PaletteResource(File);

impl GraphicInfoResource {
    pub fn load(path: &str) -> Result<Self, io::Error> {
        Ok(GraphicInfoResource(File::open(path)?))
    }
}

impl GraphicResource {
    pub fn load(path: &str) -> Result<Self, io::Error> {
        Ok(GraphicResource(File::open(path)?))
    }
}

impl PaletteResource {
    pub fn load(path: Option<&str>) -> Result<Option<Self>, io::Error> {
        match path {
            Some(path) => return Ok(Some(PaletteResource(File::open(path)?))),
            None => return Ok(None),
        }
    }
}