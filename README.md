# CompTools

[![Crates.io](https://img.shields.io/crates/v/comptools)](https://crates.io/crates/comptools)
[![docs.rs](https://img.shields.io/docsrs/comptools)](https://docs.rs/comptools)

## Create iterators using Python's list comprehesion style.

Macros for Python-like list comprehension creation of iterators.

Another Crate that tries to bring the simplicty of Python's syntax to Rust iterators.

The main macro is `iter`, and other macros are extensions of the latter.

# Examples

Below, small examples of how the macros work:
```rust
use comptools::*;

let vec: Vec<u64> = vect![x*x; for x in 0..=10; if x % 2 == 0];
assert_eq!(vec, vec![0, 4, 16, 36, 64, 100]);

let sum: u64 = sum![x*x; for x in 1..; while x*x*x < 1234];
assert_eq!(sum, 385);
```

## Contributing

Contributions are more than welcome!
