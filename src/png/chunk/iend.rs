//! Module for IEND PNG Chunk

use super::Chunk;

/// Chunk marking end of PNG file
pub struct IEND;
impl Chunk for IEND {
    fn name(&self) -> &'static str {
        "IEND"
    }
}
