# base-encode

[![crate](https://img.shields.io/crates/v/base-encode)](https://crates.io/crates/base-encode)
[![docs.rs](https://docs.rs/base-encode/badge.svg)](https://docs.rs/base-encode)

Encode and decode data from and to any base from 2 to 256.

```rust
use base_encode::{encode, decode};

let data = vec![0x27, 0x10];
encode(&data, 10) // [1, 0, 0, 0, 0]

// leading zeros are preserved
decode(&[0, 0, 2, 5, 6], 10) // [0, 0, 1, 0]
```

## Convert from and to strings

```rust
from_str("255", 10, b"0123456789").unwrap() // [0xff]

to_string(&[0xa], 2, b"OX").unwrap() // "XOXO"
```
