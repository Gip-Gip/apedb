// crc24.rs, includes functions used to compute apedb-compliant CRCs

use crc_any::CRCu32; // Use the CRC_ANY crate

pub type Crc24 = u32; // A crc24 is stored in a primitive unsigned 32 bit integer

pub static CRC24_POLY: u32 = 0x00BD80DE; // Polynomial to use
pub static CRC24_INIT: u32 = 0x00FFFFFF; // Value to initialize the CRC to, set it to all 1s
pub static CRC24_XOR: u32 = 0x00000000; // Final XOR, set to zero
pub static CRC24_REFLECT: bool = false; // Don't reflect the CRC


// crc24::compute - computes an ApeDB standard CRC24 from a slice of bytes, returns a Crc24
//
// ARGUMENTS:
//  data - a slice of bytes
pub fn compute(data: &[u8]) -> Crc24
{
    // Initialize the crc24 with the correct polynomial, init value, xor, etc. etc.
    let mut crc24 = CRCu32::create_crc(CRC24_POLY, 24, CRC24_INIT, CRC24_XOR, CRC24_REFLECT);

    crc24.digest(&data); // Generate the crc

    return crc24.get_crc(); // Return the crc
}
