use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use log::{info, warn};
use crate::data_structure::graphic::{GraphicInfo, GraphicHeader, Palette};

pub struct GraphicInfoResource(File);
pub struct GraphicResource(File);
pub struct PaletteResource(File);

impl GraphicInfoResource {
    pub fn load(path: &str) -> Result<Self, io::Error> {
        info!("Loading <GraphicInfo.bin> from {}", path);
        Ok(GraphicInfoResource(File::open(path)?))
    }
}

impl Iterator for GraphicInfoResource {
    type Item = GraphicInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 40];

        match self.0.read_exact(&mut buf) {
            Ok(_) => return Some(bincode::deserialize::<GraphicInfo>(&buf).unwrap()),
            Err(_) => return None,
        }
    }
}

impl GraphicResource {
    pub fn load(path: &str) -> io::Result<Self> {
        info!("Loading <Graphic.bin> from {}", path);
        Ok(GraphicResource(File::open(path)?))
    }

    pub fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }

    pub fn read_header(&mut self) -> Result<GraphicHeader, Box<bincode::ErrorKind>> {
        Ok(bincode::deserialize_from(&self.0)?)
    }

    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, io::Error> {
        let mut data = vec![0; size];
        self.0.read_exact(&mut data)?;

        Ok(data)
    }
}

impl PaletteResource {
    pub fn load(path: Option<&str>) -> Result<Option<Self>, io::Error> {
        info!("Loading <Palet.cgp>");
        match path {
            Some(path) => return Ok(Some(PaletteResource(File::open(path)?))),
            None => {
                warn!("Empty path of <Palet.cgp>");
                return Ok(None)
            },
        }
    }

    pub fn build(&mut self) -> Result<Palette, io::Error> {
        let mut buf = vec![];
        self.0.read_to_end(&mut buf)?;

        Ok(Palette::new(&buf))
    }
}