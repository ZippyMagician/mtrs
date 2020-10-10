use std::ops::{Add, Mul, Sub};

use crate::Matrix;

use num_traits::Num;

/// Perform addition using the `+` operator
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

            Self::from_vec(self.height, self.width, new_body)
        }
    }
}

/// Perform subtraction using the `-` operator
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

            Self::from_vec(self.height, self.width, new_body)
        }
    }
}

/// Perform multiplication using the `*` operator
impl<T: Num + Clone + Copy> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.width != other.height {
            panic!("Incorrect bounds for the two Matrices");
        }

        let mut body: Vec<Vec<T>> = vec![Vec::new(); self.height];
        let mut i = 0;

        for row in self.as_vec() {
            for col in other.cols() {
                body[i].push(
                    row.iter()
                        .zip(col.iter())
                        .map(|(&left, &right)| left * right)
                        .fold(T::zero(), |acc, x| acc + x),
                );
            }

            i += 1;
        }

        Self::from_vec(
            body.len(),
            body[0].len(),
            body.iter()
                .flat_map(|row| row.iter().map(|entry| *entry))
                .collect(),
        )
    }
}

impl<T: Num + Clone + Copy> Matrix<T> {
    /// Transposes the matrix, via mutating the original data.
    /// Does not return a new struct, instead modifies the old one.
    /// ```
    /// #[macro_use] extern crate mtrs;
    ///
    /// let mut matrix = matrix![(2, 2); 1, 2; 3, 4];
    /// matrix.transpose();
    /// assert_eq!(matrix, matrix![(2, 2); 1, 3; 2, 4]);
    /// ```
    pub fn transpose(&mut self) {
        let v = self.as_vec();
        let mut transposed = vec![Vec::with_capacity(v.len()); v[0].len()];

        for i in 0..v[0].len() {
            for j in 0..v.len() {
                transposed[i].push(v[j][i]);
            }
        }

        self.data = transposed
            .iter()
            .flat_map(|row| row.iter().map(|entry| *entry))
            .collect();

        self.height = transposed[0].len();
        self.width = transposed.len();
    }

    /// Add a scalar constant to the matrix
    pub fn scalar_add(&self, value: T) -> Self {
        Self::from_vec(
            self.height,
            self.width,
            self.data.iter().map(|x| *x + value).collect(),
        )
    }

    /// Subtract a scalar constant from the matrix
    pub fn scalar_sub(&self, value: T) -> Self {
        Self::from_vec(
            self.height,
            self.width,
            self.data.iter().map(|x| *x - value).collect(),
        )
    }

    /// Multiply a scalar constant with the matrix
    pub fn scalar_mul(&self, value: T) -> Self {
        Self::from_vec(
            self.height,
            self.width,
            self.data.iter().map(|x| *x * value).collect(),
        )
    }

    /// Divide each entry in the matrix by a scalar constant
    pub fn scalar_div(&self, value: T) -> Self {
        Self::from_vec(
            self.height,
            self.width,
            self.data.iter().map(|x| *x / value).collect(),
        )
    }

    /// Calculate the determinant of the `Matrix` (if the `Matrix` is square)
    pub fn determinant(&self) -> Option<T> {
        if self.height != self.width {
            return None;
        }

        let mut det = T::one();
        let mut total = T::one();

        let mut temp = vec![T::zero(); self.height + 1];
        let mut mat = self.data.clone();

        for i in 0..self.height {
            let mut index = i;

            while index < self.width && mat[index * self.width + i] == T::zero() {
                index += 1;
            }

            if index == self.height {
                continue;
            }

            if index != i {
                for j in 0..self.height {
                    mat.swap(index * self.width + j, i * self.width + j);
                }

                if (index - i) % 2 != 0 {
                    det = T::zero() - det;
                }
            }

            for j in 0..self.height {
                temp[j] = mat[i * self.width + j];
            }

            for j in (i + 1)..self.height {
                let diag = temp[i];
                let row = mat[j * self.width + i];

                for k in 0..self.height {
                    mat[j * self.width + k] = (diag * mat[j * self.width + k]) - (row * temp[k]);
                }

                total = total * diag;
            }
        }

        for i in 0..self.height {
            det = det * mat[i * self.width + i];
        }

        Some(det / total)
    }

    /// Calculate the inverse of `Matrix<T>`, via multiplying the reciprocal of the `determinant`
    pub fn inverse(&self) -> Option<Self> {
        if self.height != self.width {
            return None;
        }
        // The bounds are checked, so this is guaranteed to be Some
        let det = self.determinant().unwrap();
        if det == T::zero() {
            return None;
        }
        
        Some(self.scalar_mul(T::one() / det))
    }
}