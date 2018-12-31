use std::io::Cursor;
use std::str;
use std::error;

use crate::png::err::PngError;
mod ihdr;

extern crate byteorder;

type Result<T> = std::result::Result<T, Box<error::Error>>;

/// Chunk is a generic trait for PNG chunks
pub trait Chunk {
    fn name(&self) -> &'static str;
}


pub struct IEND;
impl Chunk for IEND {
    fn name(&self) -> &'static str {
        "IEND"
    }
}

pub fn parse(buf: &[u8]) -> Result<Box<Chunk>> {
    let name = str::from_utf8(&buf[..4])?;

    match name {
        "IHDR" => {
            let chunk = ihdr::parse(&buf[4..])?;
            Ok(Box::new(chunk))
        },
        "IEND" => Ok(Box::new(IEND {})),
        _      => Err(PngError::UnknownChunk.into()),
    }
}
