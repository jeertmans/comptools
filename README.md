# CompTools

[![CI Build Status][github-ci-img]][github-ci]
[![Latest Release][pypi-version-badge]][pypi-version-url]
[![Python version][pypi-python-version-badge]][pypi-version-url]
[![Documentation][docs-rtd-img]][docs-rtd-url]

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

[pypi-version-badge]: https://img.shields.io/pypi/djversions/ropey?label=Ropey
[pypi-version-url]: https://pypi.org/project/ropey/
[pypi-python-version-badge]: https://img.shields.io/pypi/pyversions/ropey
[github-ci-img]: https://github.com/jeertmans/pyropey/actions/workflows/CI.yml/badge.svg
[github-ci]: https://github.com/jeertmans/pyropey/actions?query=workflow%3Aci
[docs-rtd-url]: https://pyropey.readthedocs.io/en/latest/?badge=latest
[docs-rtd-img]: https://readthedocs.org/projects/pyropey/badge/?version=latest
