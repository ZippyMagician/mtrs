//! A library for creating, using, and printing matrices.
//! Matrices can be defined using the public struct, `mtrs::Matrix`, or the macro `matrix!`.
//! Currently the `Matrix` struct does not internally support floats. This may change in the future.
//! ```
//! #[macro_use] extern crate mtrs;
//! use mtrs::Matrix;
//!
//! fn main() {
//!     let matrix = matrix![f32; (2, 2); 1, 2; 3, 4.1];
//!     println!("{}", matrix.scalar_add(4.1));
//! }
//! ```
//! The `Matrix` struct supports addition, subtraction, and multiplication with eachother,
//! along with implementations for basic operations between a scalar value and a `Matrix`

extern crate num_traits;

mod impls;
mod math;
mod matrix;
mod size;

use num_traits::Num;

/// The main Matrix struct. Can be created in a variety of different ways.
/// ```
/// #[macro_use] extern crate mtrs;
/// use mtrs::Matrix;
///
/// // All of these create a 2 x 2 matrix.
///
/// // From a 2D vector
/// let matrix = Matrix::from_vec(2, vec![1, 2, 3, 4]);
/// // Identity matrix of i32
/// let matrix: Matrix<i32> = Matrix::identity(2);
/// // Matrix of ones
/// let matrix: Matrix<i32> = Matrix::ones(2);
/// // Matrix of zeros
/// let matrix: Matrix<i32> = Matrix::zeros(2);
/// // Matrix of `i32`s
/// let matrix = matrix![(2, 2); 1, 2, 3, 4];
/// // Matrix of `f64`s
/// let matrix = matrix![f64; (2, 2); 1, 2; 3, 4];
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<T: Num> {
    /// The height of the matrix
    height: usize,

    /// The width of the matrix
    width: usize,

    /// The main body of the matrix, stored as a 2d array
    data: Vec<T>,
}

/// A macro that can be used to define new matrices like so:
/// ```
/// #[macro_use] extern crate mtrs;
///
/// let matrix1 = matrix![(3, 3); 1, 2, 3; 4, 5, 6; 7, 8, 9;];
///
/// let matrix2 = matrix![f64; (3, 3); 1.3, 4, 6; 1, 2.5, 3.1; 1.003, 2, 9];
///
/// let mat_data: Vec<Vec<i32>> = vec![Vec::with_capacity(2); 2];
/// let matrix3 = matrix![mat_data];
/// ```
#[macro_export]
macro_rules! matrix {
    // matrix![(2, 2); 1, 2; 3, 4]
    (($height:expr,$length:expr); $($($val:expr),*);*) => {
        mtrs::Matrix::from_vec(($height, $length), {
            let mut vec = Vec::new();
            $(
                $(
                    vec.push($val);
                )*
            )*
            vec
        })
    };

    // matrix![f32; (2, 2); 1, 2; 3, 4.2]
    ($type:ty; ($height:expr,$length:expr); $($($val:expr),*);*) => {
        mtrs::Matrix::from_vec(($height, $length), {
            let mut vec = Vec::new();
            $(
                $(
                    vec.push($val as $type);
                )*
            )*
            vec
        })
    };

    // matrix![vec![vec![1, 2], vec![3, 4]]]
    ($vec:expr) => {
        mtrs::Matrix::from_vec(($vec.len(), $vec[0].len()), {
            let mut vec = Vec::new();
            for row in $vec {
                for entry in row {
                    vec.push(entry);
                }
            }

            vec
        })
    };
}
