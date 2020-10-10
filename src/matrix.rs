use crate::Matrix;

use num_traits::Num;

fn create_identity<T: Num>(size: usize) -> Vec<T> {
    let mut data = Vec::new();
    let mut offset = 0;

    for _ in 0..size {
        for x in 0..size {
            if x == offset {
                data.push(T::one());
            } else {
                data.push(T::zero())
            }
        }
        offset += 1;
    }

    data
}

impl<T: Num + Clone + Copy> Matrix<T> {
    /// Creates a new identity matrix of size `N * N`
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix: Matrix<i32> = Matrix::identity(2);
    ///
    /// assert_eq!(matrix.as_slice(), &[1, 0, 0, 1]);
    /// assert_eq!(matrix.size(), (2, 2));
    /// ```
    pub fn identity(size: usize) -> Self {
        let data = create_identity(size);

        Self {
            height: size,
            width: size,
            data,
        }
    }

    /// Creates a new matrix from a pre-given size, passing a 2d `Vec<T>`
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::from_vec(2, 2, vec![1, 2, 7, 6]);
    ///
    /// assert_eq!(matrix.as_slice(), &[1, 2, 7, 6]);
    /// ```
    pub fn from_vec(height: usize, width: usize, body: Vec<T>) -> Self {
        Self {
            height,
            width,
            data: body,
        }
    }

    /// Create a `Matrix<i32>` of size `M * N` filled with `0`s
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::zeros(2, 2);
    ///
    /// assert_eq!(matrix.as_slice(), &[0, 0, 0, 0]);
    /// assert_eq!(matrix, Matrix::from_vec(2, 2, vec![0, 0, 0, 0]));
    /// ```
    pub fn zeros(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            data: vec![T::zero(); width * height],
        }
    }

    /// Create a `Matrix<i32>` of size `M * N` filled with `1`s
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::ones(2, 2);
    ///
    /// assert_eq!(matrix.as_slice(), &[1, 1, 1, 1]);
    /// assert_eq!(matrix, Matrix::from_vec(2, 2, vec![1, 1, 1, 1]));
    /// ```
    pub fn ones(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            data: vec![T::one(); width * height],
        }
    }

    /// Returns a tuple representing the dimensions (`(height, width)`)
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix: Matrix<i32> = Matrix::ones(2, 3);
    /// assert_eq!(matrix.size(), (2, 3));
    /// ```
    pub fn size(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    /// Wrapper function for `self.data.as_slice()`
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Wrapper function for `self.data.as_mut_slice()`
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Wrapper function for `self.data.as_ptr()`
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Wrapper function for `self.data.as_mut_ptr()`
    pub fn as_mut_ptr(&mut self) -> *const T {
        self.data.as_mut_ptr()
    }

    /// Return a `Vec` representation of the Matrix
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::from_vec(2, 2, vec![2, 1, 4, 3]);
    /// assert_eq!(matrix.as_vec(), vec![vec![2, 1], vec![4, 3]]);
    /// ```
    pub fn as_vec(&self) -> Vec<Vec<T>> {
        let mut body = Vec::new();
        let sliced = self.as_slice();

        for row in 0..self.height {
            let mut row_vec = Vec::new();
            for col in 0..self.width {
                row_vec.push(sliced[row * self.width + col]);
            }

            body.push(row_vec);
        }

        body
    }

    /// Returns a single column of the Matrix
    fn get_col(&self, index: usize) -> Vec<T> {
        let mut body = Vec::new();
        let sliced = self.as_slice();

        for row in 0..self.height {
            body.push(sliced[row * self.width + index]);
        }

        body
    }

    /// Returns a `Vec` of all the columns
    pub fn cols(&self) -> Vec<Vec<T>> {
        let mut body = Vec::new();

        for i in 0..self.width {
            body.push(self.get_col(i));
        }

        body
    }
}

#[cfg(test)]
mod private_tests {
    use super::Matrix;

    #[test]
    fn test_cols() {
        let matrix1: Matrix<i32> = Matrix::identity(1);
        let matrix2: Matrix<i32> = Matrix::identity(2);
        let matrix3: Matrix<i32> = Matrix::identity(3);

        assert_eq!(matrix1.get_col(0), vec![1]);
        assert_eq!(matrix2.get_col(0), vec![1, 0]);
        assert_eq!(matrix2.get_col(1), vec![0, 1]);
        assert_eq!(matrix3.get_col(0), vec![1, 0, 0]);
        assert_eq!(matrix3.get_col(1), vec![0, 1, 0]);
        assert_eq!(matrix3.get_col(2), vec![0, 0, 1]);

        assert_eq!(matrix1.cols(), vec![vec![1]]);
        assert_eq!(matrix2.cols(), vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(
            matrix3.cols(),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]
        );
    }

    #[test]
    fn test_empty_matrix() {
        let matrix0: Matrix<i32> = Matrix::zeros(0, 0);

        assert_eq!(matrix0.size(), (0, 0));
        assert_eq!(matrix0.get_col(0), vec![]);
        assert_eq!(matrix0.as_slice(), &[]);
        assert_eq!(matrix0.scalar_add(0), matrix0);
    }
}
