# base-encode

Encode and decode data from and to any base from 2 to 256.

```rust
use base_encode::{encode, decode};

let data = vec![0x27, 0x10];
encode(&data, 10) // [1, 0, 0, 0, 0]

// leading zeros are preserved
decode(&[0, 0, 2, 5, 6], 10) // [0, 0, 1, 0]
```
