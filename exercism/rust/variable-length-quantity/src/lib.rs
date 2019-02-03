#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const MORE_BITMASK: u8 = 0x80; // (128, 10000000)
const FRST_BITMASK: u8 = 0x7f; // (127, 01111111)

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for v in values {
        // with this bitmask (0x7f): 01111111 we take the last 8 bits
        // that's gonna be the least significant bit (LSB).
        // println!(" least significant bit (LSB): {:08b}", (v & 0x7f) as u8);
        let mut bytes = vec![*v as u8 & FRST_BITMASK];

        let mut rest: u32 = v >> 7;
        while rest != 0 {
            // push in front of the vector.
            // first the bits enabled: rest & 0x7f
            // and then add the padding (|) with (0x80): 10000000
            bytes.insert(0, MORE_BITMASK | (rest as u8 & FRST_BITMASK));
            // keep reducing by 7 bits.
            rest >>= 7;
        }
        res.extend(bytes);
    }
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    println!("\n---\nbytes received: {:?}", bytes);
    let mut result = Vec::new();
    let mut current: u32 = 0;
    for (idx, byte) in bytes.iter().enumerate() {
        // checking for overflow: error: literal out of range for i32
        // if current > 0x1ffffff { return Err(Error::Overflow) }
        current.checked_mul(1 << 7).ok_or(Error::Overflow)?;
        //
        // we select the MSBs for the current number
        current <<= 7;
        //
        // ANDing with 0x7f, (byte & 0x7f) we consider only the 7 bits,
        // keeping the MBS to 0.
        current |= u32::from(byte & FRST_BITMASK);
        //
        // to persist the current number we check if the continuation bit is
        // set, ANDing with the MORE_BITMASK;
        if (byte & MORE_BITMASK) == 0 {
            // push the current number
            result.push(current);
            // reset the current number
            current = 0;
        } else
        //
        // otherwise, if we are at the end, there is still something to
        // push, so the number is incomplete.
        if idx + 1 == bytes.len() {
            return Err(Error::IncompleteNumber);
        }
    }
    Ok(result)
}
