//! Module to checksum PNG chunks

/// Struct to calculate CRC-32 on data
///
/// The struct's main purpose is to cache the
/// control byte lookup table.
pub struct Crc {
    /// Table mapping u8 -> u32 to XOR checksum with
    table: [u32; 256],
}

/// Polynomial (in reversed form) used by CRC-32
const CRC32_POLY: u32 = 0xEDB88320;

/// Generates table for CRC-32
fn gen_table() -> [u32; 256] {
    let mut table = [0; 256];

    // Iterate over all possible u8
    for i in 0..256 {
        let mut v = i as u32;
        for _ in 0..8 {
            // XOR if "popped" bit is one
            v = if v & 1 != 0 {
                (v >> 1) ^ CRC32_POLY
            } else {
                (v >> 1)
            }
        }
        table[i] = v;
    }
    table
}

/// Create new Crc struct.
pub fn new() -> Crc {
    Crc {
        table: gen_table(),
    }
}


impl Crc {
    /// Checksum the given data
    ///
    /// # Arguments
    ///
    /// * `data` - Data to checksum
    ///
    pub fn checksum(&self, data: &[u8]) -> u32 {
        // CRC-32 initializes the checksum to all 1s
        let mut sum = 0xFFFFFFFFu32;

        for byte in data {
            // "Pop" last byte and XOR with table result
            let ind = (*byte ^ (sum as u8)) as usize;
            sum = self.table[ind] ^ (sum >> 8);
        }
        !sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let crc = new();
        let checksum = crc.checksum(b"123456789");
        assert_eq!(checksum, 0xCBF43926);
    }
}
