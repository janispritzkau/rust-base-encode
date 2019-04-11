//! Functions for encoding data into any base from 2 to 256.
//!
//! ## Example
//!
//! ```rust
//! use base_encode::*;
//!
//! let data = vec![0x27, 0x10];
//! encode(&data, 10) // [1, 0, 0, 0, 0]
//! ```
//!
//! Leading zeros are preserved.
//!
//! ```rust
//! encode(&[0, 0, 128], 36) // [0, 0, 3, 14]
//! ```
//!
//! ```rust
//! decode(&[0, 2, 5, 6], 10) // [0, 1, 0]
//! ```

pub mod utils;

/// Encodes a `u8` slice to any base.
///
/// The `base` must be at least 2 and lower or equal than 256.
///
/// The return value contains the digits in the specified base.
pub fn encode(buf: &[u8], base: u16) -> Vec<u8> {
    let mut num = utils::from_bytes_be(buf);
    let mut digits = Vec::new();

    while num.len() > 1 || num[0] != 0 {
        digits.push(utils::div_rem(&mut num, base as utils::DoubleDigit) as u8);
    }

    let zeros = buf.iter().take_while(|&x| *x == 0).count();
    digits.resize(digits.len() + zeros, 0);
    digits.reverse(); digits
}

/// Decodes a base encoded `u8` slice into bytes.
///
/// The `base` must be at least 2 and lower or equal than 256.
/// You must ensure that the values are lower that the base.
pub fn decode(buf: &[u8], base: u16) -> Vec<u8> {
    let mut num = vec![0];
    let zeros = buf.iter().take_while(|&x| *x == 0).count();

    for &digit in buf {
        utils::mul(&mut num, base as utils::DoubleDigit);
        utils::add(&mut num, digit.into());
    }

    let mut bytes = vec![0; zeros];
    bytes.append(&mut utils::to_bytes_be(&num));
    bytes
}
