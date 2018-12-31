use std::io::Cursor;
use std::error;

use crate::png::err::PngError;
use super::Chunk;

use byteorder::{BigEndian, ReadBytesExt};

type Result<T> = std::result::Result<T, Box<error::Error>>;

/// IHDR Chunk containing general info about the image
pub struct IHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: ColorType,
    interlace: InterlaceMethod,
}

impl Chunk for IHDR {
    /// Return name of
    fn name(&self) -> &'static str {
        "IHDR"
    }
}

pub enum ColorType {
    Greyscale,
    TrueColor,
    IndexedColor,
    GreyscaleAlpha,
    TrueColorAlpha,
}

pub enum InterlaceMethod {
    Standard,
    Adam7,
}

impl ColorType {
    fn allowed(&self, depth: u8) -> bool {
        let mask = match *self {
            ColorType::Greyscale      => 1 | 2 | 4 | 8 | 16,
            ColorType::TrueColor      =>             8 | 16,
            ColorType::IndexedColor   => 1 | 2 | 4 | 8     ,
            ColorType::GreyscaleAlpha =>             8 | 16,
            ColorType::TrueColorAlpha =>             8 | 16,
        };
        depth & mask != 0
    }
}

pub fn parse(data: &[u8]) -> Result<IHDR> {
    let mut rdr = Cursor::new(data);

    let width = rdr.read_u32::<BigEndian>()?;
    let height = rdr.read_u32::<BigEndian>()?;
    if width == 0 || height == 0 {
        return Err(PngError::BadChunk.into());
    }

    let bit_depth = rdr.read_u8()?;

    let color_type = match rdr.read_u8()? {
        0 => ColorType::Greyscale,
        2 => ColorType::TrueColor,
        3 => ColorType::IndexedColor,
        4 => ColorType::GreyscaleAlpha,
        6 => ColorType::TrueColorAlpha,
        _ => return Err(PngError::BadChunk.into()),
    };

    if !color_type.allowed(bit_depth) {
        return Err(PngError::BadChunk.into());
    }

    let compression_method = rdr.read_u8()?;
    if compression_method != 0 {
        return Err(PngError::BadChunk.into());
    }

    let filter_method = rdr.read_u8()?;
    if filter_method != 0 {
        return Err(PngError::BadChunk.into());
    }

    let interlace = match rdr.read_u8()? {
        0 => InterlaceMethod::Standard,
        1 => InterlaceMethod::Adam7,
        _ => return Err(PngError::BadChunk.into()),
    };

    let ret = IHDR {
        width,
        height,
        bit_depth,
        color_type,
        interlace,
    };

    Ok(ret)
}
