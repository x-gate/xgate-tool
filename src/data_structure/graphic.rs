use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GraphicHeader {
    pub mark: [char; 2],
    pub version: u8,
    pub unknown: u8,
    pub width: u32,
    pub height: u32,
    pub length: u32,
}

pub struct GraphicData(Vec<u8>);

pub struct GraphicV1 {
    pub header: GraphicHeader,
    pub data: GraphicData,
}

pub struct GraphicV2 {
    pub header: GraphicHeader,
    pub palette_length: u32,
    pub data: GraphicData,
    pub palette_data: Palette
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Palette(Vec<Pixel>);

impl Palette {
    pub fn new(bytes: &[u8]) -> Self {
        let prefix = vec![
            Pixel {b: 0x00, g: 0x00, r: 0x00}, Pixel {b: 0x80, g: 0x00, r: 0x00}, Pixel {b: 0x00, g: 0x80, r: 0x00}, Pixel {b: 0x80, g: 0x80, r: 0x00},
            Pixel {b: 0x00, g: 0x00, r: 0x80}, Pixel {b: 0x80, g: 0x00, r: 0x80}, Pixel {b: 0x00, g: 0x80, r: 0x80}, Pixel {b: 0xC0, g: 0xC0, r: 0xC0},
            Pixel {b: 0xC0, g: 0xDC, r: 0xC0}, Pixel {b: 0xA6, g: 0xCA, r: 0xF0}, Pixel {b: 0xDE, g: 0x00, r: 0x00}, Pixel {b: 0xFF, g: 0x5F, r: 0x00},
            Pixel {b: 0xFF, g: 0xFF, r: 0xA0}, Pixel {b: 0x00, g: 0x5F, r: 0xD2}, Pixel {b: 0x50, g: 0xD2, r: 0xFF}, Pixel {b: 0x28, g: 0xE1, r: 0x28}
        ];
        let mut suffix = vec![
            Pixel {b: 0xF5, g: 0xC3, r: 0x96},Pixel {b: 0x1E, g: 0xA0, r: 0x5F},Pixel {b: 0xC3, g: 0x7D, r: 0x46},Pixel {b: 0x9B, g: 0x55, r: 0x1E},
            Pixel {b: 0x46, g: 0x41, r: 0x37},Pixel {b: 0x28, g: 0x23, r: 0x1E},Pixel {b: 0xFF, g: 0xFB, r: 0xF0},Pixel {b: 0x3A, g: 0x6E, r: 0xA5},
            Pixel {b: 0x80, g: 0x80, r: 0x80},Pixel {b: 0xFF, g: 0x00, r: 0x00},Pixel {b: 0x00, g: 0xFF, r: 0x00},Pixel {b: 0xFF, g: 0xFF, r: 0x00},
            Pixel {b: 0x00, g: 0x00, r: 0xFF},Pixel {b: 0xFF, g: 0x80, r: 0xFF},Pixel {b: 0x00, g: 0xFF, r: 0xFF},Pixel {b: 0xFF, g: 0xFF, r: 0xFF}    
        ];

        let mut palette = prefix;
        palette.append(&mut Self::build(bytes));
        palette.append(&mut suffix);

        Palette(palette)
    }

    fn build(bytes: &[u8]) -> Vec<Pixel> {
        let mut ret = vec![];
        bytes.chunks_exact(3).for_each(|b| ret.push(bincode::deserialize::<Pixel>(&b).unwrap()));

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bincode;

    #[test]
    fn serialize_graphic_info() {
        let graphic_info = GraphicInfo {
            id: 0, address: 0, length: 424, offset_x: -32, offset_y: -24, width: 64, height: 47, tile_east: 1, tile_south: 1, access: 1, unknown: [0, 0, 0, 0, 0], map:999
        };
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00, 0xe0, 0xff, 0xff, 0xff, 0xe8, 0xff, 0xff, 0xff, 0x40, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe7, 0x03, 0x00, 0x00];

        assert!(bytes.iter().eq(bincode::serialize(&graphic_info).unwrap().iter()));
    }

        
    #[test]
    fn deserialize_graphic_info() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00, 0xe0, 0xff, 0xff, 0xff, 0xe8, 0xff, 0xff, 0xff, 0x40, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe7, 0x03, 0x00, 0x00];
        let graphic_info = bincode::deserialize::<GraphicInfo>(&bytes).unwrap();
        let expect = GraphicInfo {
            id: 0, address: 0, length: 424, offset_x: -32, offset_y: -24, width: 64, height: 47, tile_east: 1, tile_south: 1, access: 1, unknown: [0, 0, 0, 0, 0], map:999
        };

        assert_eq!(graphic_info, expect);
    }

    #[test]
    fn serialize_graphic_header() {
        let graphic_header = GraphicHeader {
            mark: ['R', 'D'], version: 1, unknown: 16, width: 64, height: 47, length: 424,
        };
        let bytes = [0x52, 0x44, 0x01, 0x10, 0x40, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00];

        assert_eq!(bytes, bincode::serialize(&graphic_header).unwrap().as_slice());
    }

    #[test]
    fn deserialize_graphic_header() {
        let bytes = [0x52, 0x44, 0x01, 0x10, 0x40, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00];
        let graphic_header = bincode::deserialize::<GraphicHeader>(&bytes).unwrap();
        let expect = GraphicHeader {
            mark: ['R', 'D'], version: 1, unknown: 16, width: 64, height: 47, length: 424,
        };

        assert_eq!(graphic_header, expect);
    }

    #[test]
    fn serialize_pixel() {
        let pixel = Pixel {
            r: 0, g: 0, b: 0
        };
        let bytes = [0x00, 0x00, 0x00];
        
        assert_eq!(bytes, bincode::serialize(&pixel).unwrap().as_slice());
    }

    #[test]
    fn deserialize_pixel() {
        let bytes = [0x00, 0x00, 0x00];
        let pixel = bincode::deserialize::<Pixel>(&bytes).unwrap();
        let expect = Pixel {r: 0, g: 0, b: 0};

        assert_eq!(pixel, expect);
    }
}