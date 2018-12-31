//! Module to parse PNG files

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

use std::io::prelude::*;
use std::error;

mod crc;
mod chunk;

pub mod err;
use crate::png::err::PngError;

type Result<T> = std::result::Result<T, Box<error::Error>>;

/// Expected PNG header of all files
const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

/// Struct to hold PNG data
pub struct Png {
    chunks: Vec<Box<chunk::Chunk>>,
}

/// Parse PNG file
///
/// # Arguments
///
/// * `data` - data to parse PNG from
pub fn parse(buf: &mut Read) -> Result<Png> {
    // Header
    let mut header = [0; 8];
    buf.read_exact(&mut header[..])?;
    if header != PNG_HEADER {
        return Err(PngError::BadHeader.into());
    }

    let chunks = Vec::new();
    let mut png = Png { chunks };

    let chunk_crc = crc::new();
    while let Ok(size) = buf.read_u32::<BigEndian>() {
        let mut data = vec![0; (size + 4) as usize];
        buf.read_exact(&mut data)?;

        let checksum = buf.read_u32::<BigEndian>()?;
        if chunk_crc.checksum(&data) != checksum {
            return Err(PngError::BadChecksum.into());
        }

        if let Ok(png_chunk) = chunk::parse(&data) {
            println!("{}", png_chunk.name());
            png.chunks.push(png_chunk);
        } else {
            println!("Unknown chunk");
        }
    }

    Ok(png)
}
