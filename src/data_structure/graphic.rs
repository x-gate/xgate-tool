use std::io::{Cursor, Read};
use std::fmt;
use std::cmp::PartialEq;
use serde::{Serialize, Deserialize};
use byteorder::ReadBytesExt;
use bmp::{Image, Pixel as BMPPixel};
use log::{warn};

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

impl fmt::Display for GraphicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GraphicInfo {{ id: {}, address: {}, length: {}, width: {}, height: {} }}", self.id, self.address, self.length, self.width, self.height)
    }
}

impl PartialEq<GraphicHeader> for GraphicInfo {
    fn eq(&self, other: &GraphicHeader) -> bool {
        self.width == other.width && 
        self.height == other.height &&
        self.length == other.length
    }
}

impl GraphicInfo {
    pub fn valid_perimeter(&self) -> bool {
        (self.width as u64) * (self.height as u64) < u32::max_value() as u64
    }
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

impl fmt::Display for GraphicHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GraphicHeader {{ mark: {:?}, version: {}, width: {}, height: {}, length: {} }}", self.mark, self.version, self.width, self.height, self.length)
    }
}

impl PartialEq<GraphicInfo> for GraphicHeader {
    fn eq(&self, other: &GraphicInfo) -> bool {
        self.width == other.width && 
        self.height == other.height &&
        self.length == other.length
    }
}

#[derive(Debug, PartialEq)]
pub struct GraphicData(Vec<u8>);

impl GraphicData {
    pub fn decode(&self) -> Self {
        let mut cursor = Cursor::new(&self.0);
        let mut decoded = vec![];

        while let Ok(first_byte) = cursor.read_u8() {
            let data = if first_byte <= 0xaf && first_byte >= 0x80 {
                Some(cursor.read_u8().unwrap())
            } else if first_byte <= 0xef && first_byte >= 0xc0 {
                Some(0)
            } else {
                None
            };

            let num_bytes = match first_byte & 0xf0 {
                0x00|0x80|0xc0 => {
                    Some(first_byte as u32 & 0x0f)
                },
                0x10|0x90|0xd0 => {
                    Some(
                        (first_byte as u32 & 0x0f) << 8 |
                        (cursor.read_u8().unwrap() as u32)
                    )
                },
                0x20|0xa0|0xe0 => {
                    Some(
                        (first_byte as u32 & 0x0f) << 16 |
                        (cursor.read_u8().unwrap() as u32) << 8 |
                        (cursor.read_u8().unwrap() as u32)
                    )
                },
                _ => panic!("Cannot count number of bytes."),
            };

            match first_byte & 0xf0 {
                0x00|0x10|0x20 => {
                    let mut buffer = vec![0; num_bytes.unwrap() as usize];
                    cursor.read_exact(&mut buffer).unwrap();
                    decoded.append(&mut buffer);
                },
                0x80|0x90|0xa0|0xc0|0xd0|0xe0 => {
                    let mut buffer = vec![data.unwrap(); num_bytes.unwrap() as usize];
                    decoded.append(&mut buffer);
                },
                _ => panic!("Unknown encode method."),
            }
        }

        GraphicData(decoded)
    }
}

#[derive(Debug)]
pub struct Graphic {
    pub header: GraphicHeader,
    pub palette_length: Option<u32>,
    pub data: GraphicData,
    pub palette: Option<Palette>,
}

impl Graphic {
    pub fn new_v1(binary: Vec<u8>) -> Result<Self, Box<bincode::ErrorKind>> {
        let header = bincode::deserialize::<GraphicHeader>(&binary[..16])?;
        let data = GraphicData(binary[16..].to_vec());

        Ok(Self {header, palette_length: None, data, palette: None})
    }

    pub fn build_v1_image(&self, info: &GraphicInfo, palette: &Palette) -> Result<Option<Image>, std::io::Error> {
        if self.data.0.len() == 0 {
            warn!("Empty Graphic Data (id: {})", info.id);
            return Ok(None);
        }

        let mut img = Image::new(info.width, info.height);

        for (x, y) in img.coordinates() {
            let data = self.data.0[(y * info.width + x) as usize];
            let pixel = &palette.0[data as usize];
            img.set_pixel(x, info.height - y - 1, BMPPixel::new(pixel.r, pixel.g, pixel.b));
        }

        Ok(Some(img))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

#[derive(Debug)]
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
            Pixel {b: 0xF5, g: 0xC3, r: 0x96}, Pixel {b: 0x1E, g: 0xA0, r: 0x5F}, Pixel {b: 0xC3, g: 0x7D, r: 0x46}, Pixel {b: 0x9B, g: 0x55, r: 0x1E},
            Pixel {b: 0x46, g: 0x41, r: 0x37}, Pixel {b: 0x28, g: 0x23, r: 0x1E}, Pixel {b: 0xFF, g: 0xFB, r: 0xF0}, Pixel {b: 0x3A, g: 0x6E, r: 0xA5},
            Pixel {b: 0x80, g: 0x80, r: 0x80}, Pixel {b: 0xFF, g: 0x00, r: 0x00}, Pixel {b: 0x00, g: 0xFF, r: 0x00}, Pixel {b: 0xFF, g: 0xFF, r: 0x00},
            Pixel {b: 0x00, g: 0x00, r: 0xFF}, Pixel {b: 0xFF, g: 0x80, r: 0xFF}, Pixel {b: 0x00, g: 0xFF, r: 0xFF}, Pixel {b: 0xFF, g: 0xFF, r: 0xFF}    
        ];

        let mut palette = prefix;
        palette.append(&mut Self::build(bytes));
        palette.append(&mut suffix);

        Palette(palette)
    }

    fn build(bytes: &[u8]) -> Vec<Pixel> {
        let mut ret = vec![];
        bytes.chunks_exact(3).for_each(|b| ret.push(bincode::deserialize::<Pixel>(&b).unwrap()));
        // 在 Graphic V1 中，調色盤大小固定為 256 個顏色，且前後預設 32 個顏色，故此處取 224 個顏色，超出範圍者直接截斷
        ret.truncate(224);

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
    fn cmp_graphic_info_and_header() {
        let graphic_info = GraphicInfo {
            id: 0, address: 0, length: 424, offset_x: -32, offset_y: -24, width: 64, height: 47, tile_east: 1, tile_south: 1, access: 1, unknown: [0, 0, 0, 0, 0], map:999
        };
        let graphic_header = GraphicHeader {
            mark: ['R', 'D'], version: 1, unknown: 16, width: 64, height: 47, length: 424,
        };
        let other_header = GraphicHeader {
            mark: ['R', 'D'], version: 1, unknown: 16, width: 128, height: 94, length: 848,
        };

        assert!(graphic_info == graphic_header);
        assert!(graphic_header == graphic_info);
        assert!(graphic_info != other_header);
        assert!(other_header != graphic_info);
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

    #[test]
    fn new_palette() {
        let bytes = [0x85, 0xf7, 0xa4, 0x7a, 0xe2, 0x96, 0x6e, 0xcd, 0x88, 0x63, 0xb8, 0x7a, 0x57, 0xa3, 0x6c, 0x4c, 0x8e, 0x5e, 0xf0, 0xe8, 0xf5, 0xdb, 0xd7, 0xf5, 0xc5, 0xc5, 0xf5, 0xb0, 0xb4, 0xf5, 0x9a, 0xa3, 0xf5, 0x85, 0x92, 0xf5, 0x6f, 0x80, 0xf5, 0x5a, 0x6f, 0xf5, 0x52, 0x65, 0xe1, 0x49, 0x5c, 0xcd, 0x41, 0x52, 0xb9, 0x38, 0x48, 0xa5, 0x30, 0x3e, 0x91, 0x27, 0x35, 0x7d, 0x1f, 0x2b, 0x69, 0x16, 0x21, 0x55, 0xd1, 0xf1, 0xff, 0xc3, 0xe5, 0xf8, 0xb5, 0xd9, 0xf1, 0xa7, 0xcd, 0xea, 0x9a, 0xc1, 0xe4, 0x8c, 0xb5, 0xdd, 0x7e, 0xa9, 0xd6, 0x70, 0x9d, 0xcf, 0x7e, 0xb4, 0xf2, 0x70, 0xa6, 0xe5, 0x62, 0x97, 0xd8, 0x54, 0x89, 0xcb, 0x47, 0x7b, 0xbf, 0x39, 0x6d, 0xb2, 0x2b, 0x5e, 0xa5, 0x1d, 0x50, 0x98, 0xd9, 0xe2, 0xdf, 0xbe, 0xcd, 0xc8, 0xa3, 0xb8, 0xb2, 0x88, 0xa3, 0x9b, 0x6c, 0x8e, 0x85, 0x51, 0x79, 0x6e, 0x36, 0x64, 0x58, 0x1b, 0x4f, 0x41, 0xf7, 0xb1, 0xe5, 0xf1, 0x9e, 0xd1, 0xec, 0x8a, 0xbe, 0xe6, 0x77, 0xaa, 0xe0, 0x64, 0x97, 0xdb, 0x50, 0x83, 0xd5, 0x3d, 0x70, 0xcf, 0x2a, 0x5c, 0x9b, 0xda, 0xf3, 0x85, 0xcf, 0xed, 0x6f, 0xc5, 0xe6, 0x59, 0xba, 0xe0, 0x42, 0xaf, 0xda, 0x2c, 0xa4, 0xd4, 0x16, 0x9a, 0xcd, 0x00, 0x8f, 0xc7, 0x00, 0x83, 0xb8, 0x00, 0x78, 0xa9, 0x00, 0x61, 0x8c, 0x00, 0x55, 0x7d, 0x00, 0x49, 0x6e, 0x00, 0x3e, 0x5f, 0x77, 0xc2, 0xe1, 0x6d, 0xb2, 0xd1, 0x63, 0xa2, 0xc0, 0x59, 0x92, 0xb0, 0x4f, 0x81, 0xa0, 0x45, 0x71, 0x8f, 0x3c, 0x61, 0x7f, 0x32, 0x51, 0x6f, 0x28, 0x41, 0x5e, 0x1e, 0x31, 0x4e, 0x14, 0x20, 0x3e, 0x0a, 0x10, 0x2d, 0x00, 0x00, 0x1d, 0xdc, 0xf9, 0xed, 0xc6, 0xf2, 0xda, 0x8d, 0xe5, 0xb6, 0x55, 0xd9, 0x92, 0x54, 0xcc, 0x80, 0x54, 0xc0, 0x6f, 0x53, 0xb3, 0x5d, 0x52, 0xa6, 0x4b, 0x4c, 0x99, 0x45, 0x45, 0x8b, 0x3f, 0x3c, 0x7a, 0x37, 0x34, 0x68, 0x2f, 0x2b, 0x57, 0x27, 0x22, 0x45, 0x1f, 0x1a, 0x34, 0x17, 0x11, 0x22, 0x0f, 0xda, 0xe8, 0xba, 0xcf, 0xdb, 0xad, 0xc3, 0xcf, 0xa0, 0xb8, 0xc2, 0x94, 0xad, 0xb5, 0x87, 0xa1, 0xa8, 0x7a, 0x96, 0x9c, 0x6d, 0x8b, 0x8f, 0x61, 0x7f, 0x82, 0x54, 0x74, 0x76, 0x47, 0x68, 0x69, 0x3a, 0x5d, 0x5c, 0x2d, 0x52, 0x4f, 0x21, 0x46, 0x43, 0x14, 0x3b, 0x36, 0x07, 0xf6, 0xc8, 0xa0, 0xed, 0xbe, 0x95, 0xe3, 0xb4, 0x89, 0xda, 0xa9, 0x7e, 0xd0, 0x9f, 0x72, 0xc7, 0x95, 0x67, 0xbd, 0x8b, 0x5b, 0xb4, 0x81, 0x50, 0xab, 0x76, 0x45, 0xa1, 0x6c, 0x39, 0x98, 0x62, 0x2e, 0x8e, 0x58, 0x22, 0x85, 0x4d, 0x17, 0x7b, 0x43, 0x0b, 0x72, 0x39, 0x00, 0xff, 0xf8, 0xe3, 0xf8, 0xed, 0xcf, 0xf1, 0xe2, 0xba, 0xea, 0xd7, 0xa6, 0xe4, 0xcb, 0x92, 0xdd, 0xc0, 0x7e, 0xd6, 0xb5, 0x69, 0xcf, 0xaa, 0x55, 0xbf, 0x9a, 0x4a, 0xae, 0x8b, 0x40, 0x9e, 0x7b, 0x35, 0x8e, 0x6b, 0x2b, 0x7d, 0x5b, 0x20, 0x6d, 0x4c, 0x15, 0x5c, 0x3c, 0x0b, 0x4c, 0x2c, 0x00, 0xfa, 0xfa, 0xfa, 0xe0, 0xe0, 0xe0, 0xc7, 0xc7, 0xc7, 0xad, 0xad, 0xad, 0x94, 0x94, 0x94, 0x7a, 0x7a, 0x7a, 0x61, 0x61, 0x61, 0x47, 0x47, 0x47, 0x2e, 0x2e, 0x2e, 0x14, 0x14, 0x14, 0xc5, 0xd8, 0xd8, 0xaf, 0xc8, 0xc7, 0x99, 0xb9, 0xb5, 0x83, 0xa9, 0xa4, 0x6c, 0x99, 0x92, 0x56, 0x8a, 0x81, 0x40, 0x7a, 0x6f, 0xe1, 0xe0, 0xdc, 0xd5, 0xd3, 0xcd, 0xca, 0xc7, 0xbf, 0xbe, 0xbb, 0xb0, 0xb3, 0xaf, 0xa2, 0xa7, 0xa2, 0x94, 0x9c, 0x96, 0x85, 0x90, 0x8a, 0x77, 0x85, 0x7e, 0x68, 0x79, 0x71, 0x5a, 0x6e, 0x65, 0x4c, 0x62, 0x59, 0x3d, 0x57, 0x4d, 0x2f, 0x4b, 0x40, 0x20, 0xca, 0x16, 0x49, 0xc4, 0x03, 0x35, 0xb8, 0xe5, 0xf5, 0xa6, 0xdf, 0xf2, 0x95, 0xd8, 0xf0, 0x84, 0xd2, 0xed, 0x8c, 0xdb, 0xef, 0x85, 0xd0, 0xe4, 0x7d, 0xc5, 0xd9, 0x76, 0xba, 0xce, 0x6f, 0xaf, 0xc3, 0x67, 0xa5, 0xb8, 0x60, 0x9a, 0xad, 0x59, 0x8f, 0xa2, 0x51, 0x84, 0x97, 0x4a, 0x79, 0x8c, 0xde, 0xff, 0xff, 0xbe, 0xff, 0xff, 0x9f, 0xff, 0xff, 0x7f, 0xff, 0xff, 0x5f, 0xff, 0xff, 0x3f, 0xff, 0xff, 0x00, 0xfc, 0xff, 0x00, 0xec, 0xff, 0x00, 0xd8, 0xff, 0x00, 0xc5, 0xff, 0x00, 0xb2, 0xff, 0x00, 0x9e, 0xff, 0x00, 0x8b, 0xff, 0x00, 0x77, 0xff, 0x00, 0x64, 0xff, 0x00, 0x64, 0xff, 0x00, 0x56, 0xf5, 0x01, 0x48, 0xeb, 0x01, 0x3a, 0xe1, 0x01, 0x2c, 0xd7, 0x01, 0x1e, 0xcd, 0x02, 0x10, 0xc3, 0x02, 0x02, 0xa7, 0x02, 0x02, 0x95, 0x01, 0x01, 0x83, 0x01, 0x01, 0x71, 0x01, 0x01, 0x4c, 0x00, 0x00, 0x28, 0xdf, 0xdf, 0xf3, 0xc2, 0xc3, 0xe0, 0xa5, 0xa7, 0xcc, 0x88, 0x8b, 0xb9, 0x6a, 0x6f, 0xa6, 0x4d, 0x53, 0x92, 0x85, 0xaf, 0x83, 0xfe, 0xff, 0x00, 0x00, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x44, 0xc1, 0x02, 0x00, 0x94, 0xb6, 0x00, 0x00, 0x2f, 0x16, 0x2f, 0x17, 0x36, 0x74, 0x6f, 0x01, 0x36, 0x74, 0x6f, 0x01, 0x2f, 0x16, 0x94];
        let palette = Palette::new(&bytes);

        assert_eq!(256, palette.0.len());
    }

    #[test]
    fn decode_graphic_data_0x0_() {
        let graphic_data = GraphicData(vec![0x01, 0xaa]);

        assert_eq!(vec![0xaa], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0x1_() {
        let mut bytes = vec![0x11, 0x01];
        bytes.append(&mut vec![0xaa; 257]);
        let graphic_data = GraphicData(bytes);

        assert_eq!(vec![0xaa; 257], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0x2_() {
        let mut bytes = vec![0x21, 0x01, 0x01];
        bytes.append(&mut vec![0xaa; 65793]);
        let graphic_data = GraphicData(bytes);

        assert_eq!(vec![0xaa; 65793], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0x8_() {
        let graphic_data = GraphicData(vec![0x82, 0xaa]);

        assert_eq!(vec![0xaa; 2], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0x9_() {
        let graphic_data = GraphicData(vec![0x91, 0xaa, 0x01]);

        assert_eq!(vec![0xaa; 257], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0xa_() {
        let graphic_data = GraphicData(vec![0xa1, 0xaa, 0x01, 0x01]);

        assert_eq!(vec![0xaa; 65793], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0xc_() {
        let graphic_data = GraphicData(vec![0xc1]);

        assert_eq!(vec![0x00], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0xd_() {
        let graphic_data = GraphicData(vec![0xd1, 0x01]);

        assert_eq!(vec![0x00; 257], graphic_data.decode().0);
    }

    #[test]
    fn decode_graphic_data_0xe_() {
        let graphic_data = GraphicData(vec![0xe1, 0x01, 0x01]);

        assert_eq!(vec![0x00; 65793], graphic_data.decode().0);
    }
}