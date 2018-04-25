// https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs
// https://tools.ietf.org/html/rfc2898#section-5.2

extern crate crypto;
extern crate rand;
extern crate ring;

use std::str;
// https://doc.rust-lang.org/std/fs/struct.File.html
use std::fs::File;
// https://doc.rust-lang.org/std/io/trait.Read.html
// https://doc.rust-lang.org/std/io/trait.BufRead.html
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;

use self::crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use self::crypto::{aes, blockmodes, buffer, symmetriccipher};

use rand::{OsRng, RngCore};

use self::ring::{digest, pbkdf2};
use std::collections::HashMap;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

pub type Credential = [u8; CREDENTIAL_LEN];

struct PasswordDatabase {
    pbkdf2_iterations: u32,
    db_salt_component: [u8; 16],

    // Normally this would be a persistent database.
    storage: HashMap<String, Credential>,
}

enum EncError {
    WrongUsernameOrPassword,
}

#[allow(dead_code)]
impl PasswordDatabase {
    // The salt should have a user-specific component so that an attacker
    // cannot crack one password for multiple users in the database. It
    // should have a database-unique component so that an attacker cannot
    // crack the same user's password across databases in the unfortunate
    // but common case that the user has used the same password for
    // multiple systems.
    fn salt(&self, username: &str) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.db_salt_component.len() + username.as_bytes().len());
        salt.extend(self.db_salt_component.as_ref());
        salt.extend(username.as_bytes());
        salt
    }

    fn store_password(&mut self, username: &str, password: &str) {
        let salt = self.salt(username);
        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            DIGEST_ALG,
            self.pbkdf2_iterations,
            &salt,
            password.as_bytes(),
            &mut to_store,
        );
        self.storage.insert(String::from(username), to_store);
    }

    fn verify_password(
        &self,
        username: &str,
        attempted_password: &str,
    ) -> Result<(), EncError> {
        match self.storage.get(username) {
            Some(actual_password) => {
                let salt = self.salt(username);
                pbkdf2::verify(
                    DIGEST_ALG,
                    self.pbkdf2_iterations,
                    &salt,
                    attempted_password.as_bytes(),
                    actual_password,
                ).map_err(|_| EncError::WrongUsernameOrPassword)
            }

            None => Err(EncError::WrongUsernameOrPassword),
        }
    }
}

fn ring_sample() {
    // Normally these parameters would be loaded from a configuration file.
    let mut db = PasswordDatabase {
        pbkdf2_iterations: 100_000,
        db_salt_component: [
            // This value was generated from a secure PRNG.
            0xd6,
            0x26,
            0x98,
            0xda,
            0xf4,
            0xdc,
            0x50,
            0x52,
            0x24,
            0xf2,
            0x27,
            0xd1,
            0xfe,
            0x39,
            0x01,
            0x8a,
        ],
        storage: HashMap::new(),
    };

    db.store_password("alice", "@74d7]404j|W}6u");

    // An attempt to log in with the wrong password fails.
    assert!(db.verify_password("alice", "wrong password").is_err());

    // Normally there should be an expoentially-increasing delay between
    // attempts to further protect against online attacks.

    // An attempt to log in with the right password succeeds.
    assert!(db.verify_password("alice", "@74d7]404j|W}6u").is_ok());
}

// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
fn encrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor =
        aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    // Each encryption operation encrypts some data from
    // an input buffer into an output buffer. Those buffers
    // must be instances of RefReaderBuffer and RefWriteBuffer
    // (respectively) which keep track of how much data has been
    // read from or written to them.
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    // Each encryption operation will "make progress". "Making progress"
    // is a bit loosely defined, but basically, at the end of each operation
    // either BufferUnderflow or BufferOverflow will be returned (unless
    // there was an error). If the return value is BufferUnderflow, it means
    // that the operation ended while wanting more input data. If the return
    // value is BufferOverflow, it means that the operation ended because it
    // needed more space to output data. As long as the next call to the encryption
    // operation provides the space that was requested (either more input data
    // or more output space), the operation is guaranteed to get closer to
    // completing the full operation - ie: "make progress".
    //
    // Here, we pass the data to encrypt to the enryptor along with a fixed-size
    // output buffer. The 'true' flag indicates that the end of the data that
    // is to be encrypted is included in the input buffer (which is true, since
    // the input data includes all the data to encrypt). After each call, we copy
    // any output data to our result Vec. If we get a BufferOverflow, we keep
    // going in the loop since it means that there is more work to do. We can
    // complete as soon as we get a BufferUnderflow since the encryptor is telling
    // us that it stopped processing data due to not having any more data in the
    // input buffer.
    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
//
// This function is very similar to encrypt(), so, please reference
// comments in that function. In non-example code, if desired, it is possible to
// share much of the implementation using closures to hide the operation
// being performed. However, such code would make this example less clear.
fn decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

fn get_file_buffer(path: &PathBuf) -> Result<Vec<u8>, Error> {
    info!("buffer of {:?}", path);
    let file = File::open(&path)?;

    let mut buffer = Vec::new();

    let mut reader = BufReader::new(file);
    match reader.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e), // vec![0],
    }
}

pub fn sample(src: &PathBuf) -> Result<bool, Error> {
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    let key = "aaa".as_bytes();
    let iv = "bbb".as_bytes();

    match str::from_utf8(&key) {
        Ok(v) => warn!("encrypting {:?} with key: {}", src, v),
        Err(e) => warn!(
            "encrypting {:?} with generated key ({})",
            src,
            // String::from_utf8_lossy(&key),
            e
        ),
    };

    match str::from_utf8(&iv) {
        Ok(v) => warn!("encrypting {:?} with initialisation vector: {}", src, v),
        Err(e) => warn!(
            "encrypting {:?} with generated initialisation vector ({})",
            src,
            // String::from_utf8_lossy(&iv),
            e
        ),
    };

    let clear_data = get_file_buffer(src)?;
    warn!("{:?} is {:?} long", src, clear_data.len());
    let encrypted_data = encrypt(&clear_data, &key, &iv).ok().unwrap();
    let decrypted_data = decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();
    assert!(clear_data == &decrypted_data[..]);

    let mut dst: PathBuf = src.clone();
    dst.set_extension("enc");

    warn!("encryption dst is {:?}", dst);
    let mut file = File::create(dst)?;
    return match file.write_all(&encrypted_data) {
        Ok(n) => {
            info!("done with {:?}", n);
            Ok(true)
        }
        Err(e) => Err(e),
    };
}

// https://github.com/ctz/fastpbkdf2
// https://github.com/briansmith/crypto-bench
// https://github.com/briansmith/crypto-bench/blob/master/fastpbkdf2/fastpbkdf2.rs
