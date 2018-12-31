//! Module for parsing PNG chunks

use std::str;
use std::error;

use crate::png::err::PngError;
mod ihdr;
mod iend;

extern crate byteorder;

type Result<T> = std::result::Result<T, Box<error::Error>>;

/// Chunk is a generic trait for PNG chunks
pub trait Chunk {
    fn name(&self) -> &'static str;
}

/// Parse byte slice into a Chunk
pub fn parse(buf: &[u8]) -> Result<Box<Chunk>> {
    let name = str::from_utf8(&buf[..4])?;

    match name {
        "IHDR" => {
            let chunk = ihdr::parse(&buf[4..])?;
            Ok(Box::new(chunk))
        },
        "IEND" => Ok(Box::new(iend::IEND {})),
        _      => Err(PngError::UnknownChunk.into()),
    }
}
