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

use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio};

mod progressbar;
mod prompts;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

type Credential = [u8; CREDENTIAL_LEN];

fn get_file_buffer(path: &PathBuf) -> Result<Vec<u8>, Error> {
    debug!("getting buffer of {:?}", path);

    let file = File::open(&path)?;
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(file);

    match reader.read_to_end(&mut buffer) {
        Ok(usz) => {
            debug!("buffer from {:?} is {:?} bytes long", &path, usz);
            Ok(buffer)
        }
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

fn tf() -> Twofish {
    let password: String;
    loop {
        let pv = prompts::getpass::get_pass("password: ");
        let pa = match str::from_utf8(&pv) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let pv = prompts::getpass::get_pass("confirm: ");
        let pb = match str::from_utf8(&pv) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        if pa == pb {
            password = pa.to_owned();
            break;
        } else {
            warn!("passwords aren't matching");
            continue;
        };
    }

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

    debug!("initial password: {:?}", password);
    debug!("initial salt: {:?}", salt);
    debug!("initial key: {:?}", key);
    debug!("pbkdf2-ed key: {:?}", key);

    return Twofish::new_varkey(&key).unwrap();
}

#[allow(dead_code)]
pub fn cipher(src: &PathBuf) -> Result<PathBuf, Error> {
    let f_buf: Vec<u8> = get_file_buffer(src)?;
    let f_buf: &[u8] = &f_buf;

    let twofish = tf();
    let mut encrypted = Vec::new();
    let mut pb = progressbar::simple(((f_buf.len() as f64) / 16.0).ceil() as u64);

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

    debug!("encryption dst is {:?}", dst);
    let mut file = File::create(&dst)?;
    return match file.write_all(&encrypted) {
        Ok(n) => {
            info!("done with {:?}", n);
            Ok(dst)
        }
        Err(e) => Err(e),
    };
}

#[allow(dead_code)]
pub fn decipher(src: &PathBuf) -> Result<PathBuf, Error> {
    let f_buf: Vec<u8> = get_file_buffer(src)?;
    let f_buf: &[u8] = &f_buf;

    let twofish = tf();
    let mut decrypted = Vec::new();
    let mut pb = progressbar::simple(((f_buf.len() as f64) / 16.0).ceil() as u64);

    for chunk in f_buf.chunks(16) {
        let enc = GenericArray::from_slice(chunk);
        let mut buf = enc.clone();

        twofish.decrypt_block(&mut buf);

        let mut plain = buf.clone();
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_slice
        decrypted.extend_from_slice(&plain);
        pb.inc();
    }

    assert_eq!(f_buf.len(), decrypted.len());
    pb.finish();

    let mut dst: PathBuf = src.clone();
    dst.set_extension("foo");

    debug!("decryption dst is {:?}", dst);
    let mut file = File::create(&dst)?;
    return match file.write_all(&decrypted) {
        Ok(n) => {
            info!("done with {:?}", n);
            Ok(dst)
        }
        Err(e) => Err(e),
    };
}

pub fn symmetric(src: &PathBuf, out: &PathBuf) -> Result<PathBuf, Error> {
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
        .collect();
    let mut dst = out.clone();
    dst.set_extension("gpg");
    // https://doc.rust-lang.org/std/process/struct.Command.html
    match Command::new("gpg")
        .arg("--symmetric")
        .args(&["--cipher-algo", "AES256"])
        .args(&["--compress-level", "6"])
        .args(&["--compress-algo", "ZLIB"])
        .args(&["--output", dst.to_str().unwrap()])
        .arg(src.to_str().unwrap())
        .current_dir(src.parent().expect("src must be a file"))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .env_clear()
        .envs(&filtered_env)
        .spawn()
    {
        Ok(child) => {
            let output = child.wait_with_output().expect("failed to wait on child");
            info!("Child has finished its execution!");
            assert!(output.status.success());
            return Ok(dst);
        }
        Err(e) => {
            error!("ls command didn't start");
            return Err(e);
        }
    }
}
