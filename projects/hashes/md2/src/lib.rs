//! The [MD2][1] hash function.
//!
//! [1]: https://en.wikipedia.org/wiki/MD2_(cryptography)

// Range loops are preferred for reading simplicity
#![cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]

#![no_std]
#[macro_use]
extern crate digest;
extern crate block_buffer;

pub use digest::Digest;
use block_buffer::{BlockBuffer128, Pkcs7};
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::U16;

mod consts;

type Block = [u8; 16];

#[derive(Clone)]
struct Md2State {
    x: [u8; 48],
    checksum: [u8; 16],
}

impl Default for Md2State {
    fn default() -> Self {
        Self { x: [0; 48], checksum: [0; 16] }
    }
}

/// The MD2 hasher
#[derive(Clone, Default)]
pub struct Md2 {
    buffer: BlockBuffer128,
    state: Md2State,
}

impl Md2State {
    fn process_block(&mut self, input: &Block) {
        // Update state
        for j in 0..16 {
            self.x[16 + j] = input[j];
            self.x[32 + j] = self.x[16 + j] ^ self.x[j];
        }

        let mut t = 0u8;
        for j in 0..18u8 {
            for k in 0..48 {
                self.x[k] ^= consts::S[t as usize];
                t = self.x[k];
            }
            t = t.wrapping_add(j);
        }

        // Update checksum
        let mut l = self.checksum[15];
        for j in 0..16 {
            self.checksum[j] ^= consts::S[(input[j] ^ l) as usize];
            l = self.checksum[j];
        }
    }
}

impl Md2 {
    pub fn new() -> Md2 {
        Default::default()
    }

    fn finalize(&mut self) {
        let buf = self.buffer.pad_with::<Pkcs7>();
        self.state.process_block(buf);
        let checksum = self.state.checksum;
        self.state.process_block(&checksum);
    }
}


impl digest::BlockInput for Md2 {
    type BlockSize = U16;
}

impl digest::Input for Md2 {
    fn process(&mut self, input: &[u8]) {
        let self_state = &mut self.state;
        self.buffer.input(input, |d: &Block| {
            self_state.process_block(d);
        });
    }
}

impl digest::FixedOutput for Md2 {
    type OutputSize = U16;

    fn fixed_result(mut self) -> GenericArray<u8, Self::OutputSize> {
        self.finalize();

        GenericArray::clone_from_slice(&self.state.x[0..16])
    }
}

impl_opaque_debug!(Md2);
