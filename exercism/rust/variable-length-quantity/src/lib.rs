#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for v in values {
        // with this bitmask (0x7f): 01111111 we take the last 8 bits
        // that's gonna be the least significant bit (LSB).
        // println!(" least significant bit (LSB): {:08b}", (v & 0x7f) as u8);
        let mut bytes = vec![(v & 0x7f) as u8];

        let mut rest: u32 = v >> 7;
        while rest != 0 {
            // push in front of the vector.
            // first the bits enabled: rest & 0x7f
            // and then add the padding (|) with (0x80): 10000000
            bytes.insert(0, (0x80 | (rest & 0x7f)) as u8);
            // keep reducing by 7 bits.
            rest >>= 7;
        }
        res.extend(bytes);
    }
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}
