// https://github.com/RustCrypto/traits
// https://github.com/fizyk20/generic-array
// http://fizyk20.github.io/generic-array/generic_array/

// https://github.com/ctz/fastpbkdf2
// https://github.com/briansmith/crypto-bench
// https://github.com/briansmith/crypto-bench/blob/master/fastpbkdf2/fastpbkdf2.rs

extern crate block_cipher_trait;
extern crate crypto;
extern crate ring;
extern crate twofish;

// https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs
// https://tools.ietf.org/html/rfc2898#section-5.2
// https://github.com/RustCrypto/block-ciphers

use std::str;
// https://doc.rust-lang.org/std/fs/struct.File.html
use std::fs::File;
// https://doc.rust-lang.org/std/io/trait.Read.html
// https://doc.rust-lang.org/std/io/trait.BufRead.html
// use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;

// use rand::{OsRng, RngCore};
// use self::crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
// use self::crypto::{aes, blockmodes, buffer, symmetriccipher};

use self::ring::{digest, pbkdf2};

// http://fizyk20.github.io/generic-array/generic_array/
// https://github.com/fizyk20/generic-array
// https://github.com/RustCrypto/traits
use self::block_cipher_trait::generic_array::GenericArray;
use self::block_cipher_trait::BlockCipher;

use self::twofish::Twofish;

mod progressbar;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

type Credential = [u8; CREDENTIAL_LEN];

fn get_file_buffer(path: &PathBuf) -> Result<Vec<u8>, Error> {
    debug!("getting buffer of {:?}", path);

    let file = File::open(&path)?;
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(file);

    match reader.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

fn salt(component: Vec<u8>, input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(component.len() + input.as_bytes().len());
    output.extend(component);
    output.extend(input.as_bytes());
    output
}

// This value was generated from a secure PRNG.
fn component() -> Vec<u8> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    vec![
        0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
        0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a,
    ]
}

pub fn cipher(src: &PathBuf) -> Result<PathBuf, Error> {
    let password: String = String::from("foobar");
    let pbkdf2_iterations: u32 = 100_000;
    let salt = salt(component(), "taker");
    let mut key: Credential = [0u8; CREDENTIAL_LEN];
    
    pbkdf2::derive(
        DIGEST_ALG,
        pbkdf2_iterations,
        &salt,
        password.as_bytes(),
        &mut key,
    );

    let twofish = Twofish::new_varkey(&key).unwrap();
    let f_buf: Vec<u8> = get_file_buffer(src)?;
    let f_buf: &[u8] = &f_buf;
    
    debug!("initial password: {:?}", password);
    debug!("initial salt: {:?}", salt);
    debug!("initial key: {:?}", key);
    debug!("pbkdf2-ed key: {:?}", key);
    debug!("buffer from file is {:?} bytes long", f_buf.len());

    let mut encrypted = Vec::new();
    let mut pb = progressbar::simple(((f_buf.len() as f64)/16.0).ceil() as u64);
    
    for chunk in f_buf.chunks(16) {
        let plain = GenericArray::from_slice(chunk);
        let mut buf = plain.clone();

        twofish.encrypt_block(&mut buf);
        let mut cipher = buf.clone();
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_slice
        encrypted.extend_from_slice(&cipher);

        twofish.decrypt_block(&mut cipher);
        assert_eq!(plain, &cipher);
        pb.inc();
    }
    
    assert_eq!(f_buf.len(), encrypted.len());
    pb.finish();

    let mut dst: PathBuf = src.clone();
    dst.set_extension("enc");

    warn!("encryption dst is {:?}", dst);
    let mut file = File::create(&dst)?;
    return match file.write_all(&encrypted) {
        Ok(n) => {
            info!("done with {:?}", n);
            Ok(dst)
        }
        Err(e) => Err(e),
    };
}
