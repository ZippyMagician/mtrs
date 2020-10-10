# mtrs
[![Crates.io](https://img.shields.io/crates/v/inferno.svg)](https://crates.io/crates/mtrs)
[![Documentation](https://docs.rs/inferno/badge.svg)](https://docs.rs/mtrs/)
[![Codecov](https://codecov.io/github/jonhoo/inferno/coverage.svg?branch=master)](https://codecov.io/gh/ZippyMagician/mtrs)
A library for handling mathematical matrices in Rust

## Quick example
```rs
#[macro_use]
extern crate mtrs;

use mtrs::Matrix;

fn main() {
    let mat1 = Matrix::identity(2);
    let mat2 = matrix![(2, 2); 1, 2; 3, 4];

    assert_eq!(mat1 * mat2, mat2);
}
```