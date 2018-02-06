// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "with-bench", feature(test))]

extern crate rand;
extern crate rustc_serialize as serialize;
extern crate time;
extern crate libc;

#[cfg(all(test, feature = "with-bench"))]
extern crate test;

pub mod aead;
pub mod aes;
pub mod aes_gcm;
pub mod aessafe;
pub mod blockmodes;
pub mod buffer;
mod cryptoutil;
pub mod digest;
pub mod ghash;
pub mod mac;
mod simd;
mod step_by;
pub mod symmetriccipher;
pub mod util;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod aesni;
