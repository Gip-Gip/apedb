// crc24.rs, includes functions used to compute apedb-compliant CRCs

use crc_any::CRCu32; // Use the CRC_ANY crate

const CRC24_POLY: u32 = 0x00BD80DE; // Polynomial to use
const CRC24_INIT: u32 = 0x00FFFFFF; // Value to initialize the CRC to, set it to all 1s
const CRC24_XOR: u32 = 0x00000000; // Final XOR, set to zero
const CRC24_REFLECT: bool = false; // Don't reflect the CRC

// dbcrc24::ApeCrc24 - native crc implementation
//
pub struct ApeCrc24
{
    crc24: u32, // The CRC value
}

impl ApeCrc24
{
    // new - computes an ApeDB standard CRC24 from a slice of bytes, returns a Crc24
    //
    // ARGUMENTS:
    //  data - a slice of bytes
    pub fn new(data: &[u8]) -> ApeCrc24
    {
        // Initialize the crc24 with the correct polynomial, init value, xor, etc. etc.
        let mut crc24 = CRCu32::create_crc(CRC24_POLY, 24, CRC24_INIT, CRC24_XOR, CRC24_REFLECT);

        crc24.digest(&data); // Generate the crc

        return ApeCrc24
        {
            crc24: crc24.get_crc(), // Return the crc
        }
    }

    // crc24::to_be_bytes - converts an ApeDB standard CRC24 to a be byte array
    //
    pub fn to_be_bytes(&self) -> [u8; 3]
    {
        let crc_bytes = &self.crc24.to_be_bytes()[1..4];
        
        return crc_bytes.try_into().expect("CRC slicing gone wrong! You shouldn't see this!");
    }

    // crc24::verify - verify a slice of data. The data is assumed to have the crc appended to the end
    //
    pub fn verify(data: &[u8]) -> bool
    {
        let crc24 = ApeCrc24::new(data);

        return crc24.crc24 == 0;
    }
}



// Tests!
//

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    // Test to see if a crc can detect valid data
    fn test_crc24_valid_data()
    {
        let hello = b"Hello World!";

        let crc24 = ApeCrc24::new(hello);

        let mut data = Vec::<u8>::new();

        data.extend_from_slice(hello);
        data.extend_from_slice(&crc24.to_be_bytes());

        assert_eq!(ApeCrc24::verify(&data), true);
    }

    #[test]
    // Test to see if a crc can detect invalid data
    fn test_crc24_invalid_data()
    {
        let hello = b"Hello World!";

        let crc24 = ApeCrc24::new(hello);

        let mut data = Vec::<u8>::new();

        data.extend_from_slice(hello);

        data[3] = 0xFF; // Intentionally modify the string to make it invalid

        data.extend_from_slice(&crc24.to_be_bytes());

        assert_eq!(ApeCrc24::verify(&data), false);
    }
}