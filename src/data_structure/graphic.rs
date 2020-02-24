pub struct GraphicInfo {
    pub id: u32,
    pub address: u32,
    pub length: u32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: u32,
    pub height: u32,
    pub tile_east: u8,
    pub tile_south: u8,
    pub access: u8,
    pub unknown: [u8; 5],
    pub map: u32,
}

pub struct GraphicHeader {
    pub mark: [char; 2],
    pub version: u8,
    pub unknown: u8,
    pub width: u32,
    pub height: u32,
    pub length: u32,
}

pub struct GraphicV1 {
    pub header: GraphicHeader,
    pub data: Vec<u8>
}

pub struct GraphicV2 {
    pub header: GraphicHeader,
    pub palette_length: u32,
    pub data: Vec<u8>,
    pub palette_data: Vec<u8>
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Palette(Vec<Pixel>)