use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::ops::Index;

use crate::size::Size;
use crate::Matrix;

use num_traits::Num;

/// Pretty print of the `Matrix` via this impl
impl<T> Display for Matrix<T>
where
    T: Num + Clone + Copy + Display,
{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for row in 1..=self.height {
            for index in ((row - 1) * self.height)..(self.width * row) {
                write!(fmt, "{} ", self.data[index])?;
            }

            writeln!(fmt)?;
        }

        Ok(())
    }
}

/// Allows for the indexing of `Matrix`
impl<T: Num, S: Size> Index<S> for Matrix<T> {
    type Output = T;

    fn index(&self, pos: S) -> &Self::Output {
        let (h, w) = pos.dim();
        &self.data[h * self.width + w]
    }
}

/// Implements the `From<Matrix<T>>` trait for `Vec<T>`
impl<T: Num + Clone + Copy> From<Matrix<T>> for Vec<T> {
    fn from(mat: Matrix<T>) -> Self {
        mat.as_slice().to_vec()
    }
}
