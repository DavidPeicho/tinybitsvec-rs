# TinyBitVec

Tiny vector of bits using a `u32` slice as storage.

## Example

Basic example:

```rust
use tinybitvec::BitVec;

// 10 bits reserved
let mut bits = BitVec::default(10);
bits.push(false);
bits.push(true);
bits.push(false);

println("{:?}", bits[1]) // true
println("{:?}", bits.get(1)) // Some(true)

bits.unset(1);
println("{:?}", bits[1]) // false
```

Immutable and mutable slice types:

```rust
use tinybitvec::BitVec;

let bits = BitVec::from([false, true, false, false][..]);

let slice = bits.as_slice();
println!("{:?}", slice.len()) // 4

let slice = slice.slice(0..3);
println!("{:?}", slice.iter.collect::<Vec<bool>>()) // [false, true, false]
```
