#![allow(dead_code)]

pub type Digit = u32;
pub type DoubleDigit = u64;
pub const BYTES: usize = 4;
pub const BITS: usize = BYTES * 8;

pub fn from_bytes_le(bytes: &[u8]) -> Vec<Digit> {
    bytes.chunks(BYTES).map(|c| {
        c.iter().rev().fold(0, |a, &x| (a << 8) | x as Digit)
    }).collect()
}

pub fn from_bytes_be(bytes: &[u8]) -> Vec<Digit> {
    let mut bytes = bytes.to_vec();
    bytes.reverse();
    from_bytes_le(&bytes)
}

pub fn to_bytes_le(num: &[Digit]) -> Vec<u8> {
    let mut bytes: Vec<_> = num.iter().map(|c| {
        (0..BYTES).map(move |i| (c >> (i * 8)) as u8 & 0xff)
    }).flatten().collect();
    bytes.truncate(bytes.iter().rposition(|x| *x != 0).unwrap_or(0) + 1);
    bytes
}

pub fn to_bytes_be(num: &[Digit]) -> Vec<u8> {
    let mut bytes = to_bytes_le(num);
    bytes.reverse();
    bytes
}

pub fn add(num: &mut Vec<Digit>, x: DoubleDigit) {
    let mut carry = x;
    for d in num.iter_mut() {
        carry += *d as DoubleDigit;
        *d = carry as Digit;
        carry >>= BITS;
        if carry == 0 { break }
    }
    if carry != 0 { num.push(carry as Digit) }
}

pub fn mul(num: &mut Vec<Digit>, x: DoubleDigit) {
    let mut carry = 0;
    for d in num.iter_mut() {
        carry += *d as DoubleDigit * x;
        *d = carry as Digit;
        carry >>= BITS;
    }
    if carry != 0 { num.push(carry as Digit) }
}

pub fn div_rem(num: &mut Vec<Digit>, x: DoubleDigit) -> Digit {
    let mut rem = 0;
    for d in num.iter_mut().rev() {
        let a = *d as DoubleDigit | (rem << BITS);
        *d = (a / x) as Digit;
        rem = a % x;
    }
    num.truncate(num.iter().rposition(|x| *x != 0).unwrap_or(0) + 1);
    rem as Digit
}
