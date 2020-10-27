use std::iter::Sum;
use std::ops::*;

use crate::Matrix;

use num_traits::Num;

/// Implements addition between `Matrix<T>` and `Matrix<T>`
impl<T: Num + Clone + Copy> Add for Matrix<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.size() != other.size() {
            unimplemented!();
        } else {
            let mut new_body = Vec::new();
            let other_slice = other.as_slice();

            for (index, i) in self.data.iter().enumerate() {
                new_body.push(*i + other_slice[index]);
            }

            Self::from_vec(self.size(), new_body)
        }
    }
}

/// Implements subtraction between `Matrix<T>` and `Matrix<T>`
impl<T: Num + Clone + Copy> Sub for Matrix<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.size() != other.size() {
            unimplemented!();
        } else {
            let mut new_body = Vec::new();
            let other_slice = other.as_slice();

            for (index, i) in self.data.iter().enumerate() {
                new_body.push(*i - other_slice[index]);
            }

            Self::from_vec(self.size(), new_body)
        }
    }
}

/// Implements multiplication between `Matrix<T>` and `Matrix<T>`
impl<T: Num + Clone + Copy + Sum> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.width != other.height {
            panic!("Incorrect bounds for the two Matrices");
        }

        let mut body: Vec<Vec<T>> = vec![Vec::new(); self.height];

        for (i, row) in self.as_vec().iter().enumerate() {
            for col in other.cols() {
                body[i].push(
                    row.iter()
                        .zip(col.iter())
                        .map(|(&left, &right)| left * right)
                        .sum::<T>(),
                );
            }
        }

        Self::from_vec(
            (body.len(), body[0].len()),
            body.iter().flat_map(|row| row.iter().copied()).collect(),
        )
    }
}

/// Implements addition between `Matrix<T>` and `T`
impl<T: Num + Clone + Copy> Add<T> for Matrix<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        self.scalar_add(rhs)
    }
}

/// Implements subtraction between `Matrix<T>` and `T`
impl<T: Num + Clone + Copy> Sub<T> for Matrix<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        self.scalar_sub(rhs)
    }
}

/// Implements multiplication between `Matrix<T>` and `T`
impl<T: Num + Clone + Copy> Mul<T> for Matrix<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.scalar_mul(rhs)
    }
}

/// Implements division between `Matrix<T>` and `T`
impl<T: Num + Clone + Copy> Div<T> for Matrix<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self.scalar_div(rhs)
    }
}

#[cfg(test)]
mod math_tests {
    use crate::Matrix;

    #[test]
    fn test_ops() {
        let matrix: Matrix<f32> = Matrix::from_slice((1, 3), &[1.0, 4.0, 7.0]);

        assert_eq!(
            matrix.clone() + 3.0,
            Matrix::from_slice((1, 3), &[4.0, 7.0, 10.0])
        );
        assert_eq!(
            matrix.clone() - 1.0,
            Matrix::from_slice((1, 3), &[0.0, 3.0, 6.0])
        );
        assert_eq!(
            matrix.clone() / 2.0,
            Matrix::from_slice((1, 3), &[0.5, 2.0, 3.5])
        );
        assert_eq!(
            matrix.clone() * 2.0,
            Matrix::from_slice((1, 3), &[2.0, 8.0, 14.0])
        );
    }
}
