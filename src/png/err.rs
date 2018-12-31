//! Module for various errors used when parsing PNGs

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
/// Errors used when parsing
pub enum PngError {
    BadChecksum,
    BadHeader,
    UnknownChunk,
    BadChunk,
}

impl PngError {
    /// Return message used for Display & Error traits
    fn msg(&self) -> &str {
        match *self {
            PngError::BadChecksum  => "Bad checksum",
            PngError::BadHeader    => "Bad header",
            PngError::UnknownChunk => "Unknown chunk type",
            PngError::BadChunk     => "Malformed chunk",
        }
    }
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg())
    }
}

impl error::Error for PngError {
    fn description(&self) -> &str {
        self.msg()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
