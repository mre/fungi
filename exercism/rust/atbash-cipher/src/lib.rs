// atbash: ((25 - ((c as u8) - 97)) + 97) as char
fn ciph(c: char) -> Option<char> {
    match c {
        'a'...'z' => Some((b'a' + b'z' - c as u8) as char),
        '0'...'9' => Some(c),
        _ => None,
    }
}

// https://doc.rust-lang.org/std/vec/struct.Vec.html
// chunks: pub fn chunks(&self, chunk_size: usize) -> Chunks<T>
//
// Returns an iterator over chunk_size elements of the slice at a time,
// starting at the beginning of the slice.
//
// The chunks are slices and do not overlap. If chunk_size does not
// divide the length of the slice, then the last chunk will not have
// length chunk_size.
//
// See chunks_exact for a variant of this iterator that returns chunks
// of always exactly chunk_size elements, and rchunks for the same
// iterator but starting at the end of the slice of the slice.

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let chars: Vec<char> = plain.to_lowercase().chars().collect::<Vec<char>>();
    let ciphc: Vec<char> = chars.iter().filter_map(|c| ciph(*c)).collect::<Vec<char>>();
    let chunks: std::slice::Chunks<char> = ciphc.chunks(5);
        
    return chunks.map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().join(&" ");
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let chars: Vec<char> = cipher.chars().collect::<Vec<char>>();
    let ciphc: Vec<char> = chars.iter().filter_map(|c| ciph(*c)).collect::<Vec<char>>();
    let chunks: std::slice::Chunks<char> = ciphc.chunks(5);
        
    return chunks.map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().join(&"");
}
