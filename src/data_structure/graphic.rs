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

pub struct GraphicV1 {
    pub header: GraphicHeader,
    pub data: Vec<u8>
}

pub struct GraphicV2 {
    pub header: GraphicHeader,
    pub palette_length: u32,
    pub data: Vec<u8>,
    pub palette_data: Palette
}

#[derive(Serialize, Deserialize)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Palette(Vec<Pixel>);

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
    fn deserialize_graphic_header() {
        let graphic_header = GraphicHeader {
            mark: ['R', 'D'], version: 1, unknown: 16, width: 64, height: 47, length: 424,
        };
        let bytes = [0x52, 0x44, 0x01, 0x10, 0x40, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00];

        assert_eq!(bytes, bincode::serialize(&graphic_header).unwrap().as_slice());
    }
    
    #[test]
    fn serialize_graphic_header() {
        let bytes = [0x52, 0x44, 0x01, 0x10, 0x40, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0xa8, 0x01, 0x00, 0x00];
        let graphic_header = bincode::deserialize::<GraphicHeader>(&bytes).unwrap();
        let expect = GraphicHeader {
            mark: ['R', 'D'], version: 1, unknown: 16, width: 64, height: 47, length: 424,
        };

        assert_eq!(graphic_header, expect);
    }
}