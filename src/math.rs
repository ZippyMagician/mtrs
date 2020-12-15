use crate::Matrix;

use num_traits::Num;

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
            for row in &v {
                transposed[i].push(row[i]);
            }
        }

        self.data = transposed
            .iter()
            .flat_map(|row| row.iter().copied())
            .collect();
        self.height = transposed[0].len();
        self.width = transposed.len();
    }

    /// Add a scalar constant to the matrix
    /// ```
    /// #[macro_use] extern crate mtrs;
    /// use mtrs::Matrix;
    ///
    /// let m1 = Matrix::identity(2);
    ///
    /// assert_eq!(m1.scalar_add(2), matrix![(2, 2); 3, 2; 2, 3]);
    /// ```
    pub fn scalar_add(&self, value: T) -> Self {
        Self::from_vec(self.size(), self.data.iter().map(|x| *x + value).collect())
    }

    /// Subtract a scalar constant from the matrix
    /// ```
    /// #[macro_use] extern crate mtrs;
    /// use mtrs::Matrix;
    ///
    /// let m1 = Matrix::identity(2);
    ///
    /// assert_eq!(m1.scalar_sub(2), matrix![(2, 2); -1, -2; -2, -1]);
    /// ```
    pub fn scalar_sub(&self, value: T) -> Self {
        Self::from_vec(self.size(), self.data.iter().map(|x| *x - value).collect())
    }

    /// Multiply a scalar constant with the matrix
    /// ```
    /// #[macro_use] extern crate mtrs;
    /// use mtrs::Matrix;
    ///
    /// let m1 = Matrix::identity(2);
    ///
    /// assert_eq!(m1.scalar_mul(3), matrix![(2, 2); 3, 0; 0, 3]);
    /// ```
    pub fn scalar_mul(&self, value: T) -> Self {
        Self::from_vec(self.size(), self.data.iter().map(|x| *x * value).collect())
    }

    /// Divide each entry in the matrix by a scalar constant
    /// ```
    /// #[macro_use] extern crate mtrs;
    /// use mtrs::Matrix;
    ///
    /// let m1 = matrix![(2, 2); 4, 6; 8, 10];
    ///
    /// assert_eq!(m1.scalar_div(2), matrix![(2, 2); 2, 3; 4, 5]);
    /// ```
    pub fn scalar_div(&self, value: T) -> Self {
        Self::from_vec(self.size(), self.data.iter().map(|x| *x / value).collect())
    }

    /// Calculate the determinant of the `Matrix` (if the `Matrix` is square)
    /// ```
    /// #[macro_use] extern crate mtrs;
    ///
    /// let matrix = matrix![(4, 4); 1, 0, 2, -1; 3, 0, 0, 5; 2, 1, 4, -3; 1, 0, 5, 0];
    ///
    /// assert_eq!(matrix.determinant(), Some(30));
    /// ```
    /// # Failure
    /// Fails if the matrix is not square
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
                    // Negate det
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
    /// ```
    /// #[macro_use] extern crate mtrs;
    ///
    /// let matrix = matrix![f32; (2, 2); -1, 1.5; 1, -1];
    ///
    /// assert_eq!(matrix.inverse().expect("Could not take inverse"), matrix![f32; (2, 2); 2, -3; -2, 2]);
    /// ```
    /// # Failure
    /// Fails if the matrix is not invertible (that is, it is not square __or__ the determinant is `0`)
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant()?;
        if det.is_zero() {
            None
        } else {
            Some(self.scalar_div(det))
        }
    }
}
