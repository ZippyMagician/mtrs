# mtrs
[![Crates.io](https://img.shields.io/crates/v/mtrs.svg)](https://crates.io/crates/mtrs)
[![Documentation](https://docs.rs/mtrs/badge.svg)](https://docs.rs/mtrs/)
[![Codecov](https://codecov.io/github/ZippyMagician/mtrs/coverage.svg?branch=master)](https://codecov.io/gh/ZippyMagician/mtrs)
[![Build Status](https://travis-ci.org/ZippyMagician/mtrs.svg?branch=master)](https://travis-ci.org/ZippyMagician/mtrs)<br>
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