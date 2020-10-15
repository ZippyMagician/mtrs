use crate::size::Size;
use crate::Matrix;

use std::io;

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

macro_rules! new {
    ($height:ident, $width:ident, $body:expr) => {
        Matrix {
            height: $height,
            width: $width,
            data: $body,
        }
    };
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
        new!(size, size, create_identity(size))
    }

    /// Creates a new matrix from a pre-given size, passing a 2d `Vec<T>`
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::from_vec((2, 2), vec![1, 2, 7, 6]);
    ///
    /// assert_eq!(matrix.as_slice(), &[1, 2, 7, 6]);
    /// ```
    pub fn from_vec<S: Size>(size: S, body: Vec<T>) -> Self {
        let (height, width) = size.dim();
        new!(height, width, body)
    }

    /// Creates a new matrix from a slice
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::from_slice((2, 2), &[1, 2, 7, 6]);
    ///
    /// assert_eq!(matrix.as_slice(), &[1, 2, 7, 6]);
    /// assert_eq!(matrix[(1, 0)], 7);
    /// ```
    pub fn from_slice<S: Size>(size: S, body: &[T]) -> Self {
        let (height, width) = size.dim();
        new!(height, width, body.to_vec())
    }

    /// Create a `Matrix<T>` of size `M * N` filled with `0`s
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::zeros(2);
    ///
    /// assert_eq!(matrix.as_slice(), &[0, 0, 0, 0]);
    /// assert_eq!(matrix, Matrix::from_vec(2, vec![0, 0, 0, 0]));
    /// ```
    pub fn zeros<S: Size>(size: S) -> Self {
        let (height, width) = size.dim();
        new!(height, width, vec![T::zero(); width * height])
    }

    /// Create a `Matrix<T>` of size `M * N` filled with `1`s
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::ones(2);
    ///
    /// assert_eq!(matrix.as_slice(), &[1, 1, 1, 1]);
    /// assert_eq!(matrix, Matrix::from_vec(2, vec![1, 1, 1, 1]));
    /// ```
    pub fn ones<S: Size>(size: S) -> Self {
        let (height, width) = size.dim();
        new!(height, width, vec![T::one(); width * height])
    }

    /// Returns a tuple representing the dimensions (`(height, width)`)
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix: Matrix<i32> = Matrix::ones((2, 3));
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
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Zero out the matrix
    ///
    /// Safety: Only use when the values inside the matrix can be safely zeroed
    #[inline]
    pub unsafe fn erase(&mut self) {
        std::ptr::write_bytes(self.as_mut_ptr(), 0, self.data.len());
    }

    /// Return a `Vec` representation of the Matrix
    /// ```
    /// use mtrs::Matrix;
    ///
    /// let matrix = Matrix::from_vec(2, vec![2, 1, 4, 3]);
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
    /// Returns a `Vec` of all the columns
    /// ```
    /// extern crate mtrs;
    /// use mtrs::matrix;
    ///
    /// let mat = matrix![i32; (3, 2); 1, 2; 3, 4; 5, 6];
    ///
    /// assert_eq!(mat.get_col(0), Some(vec![1, 3, 5]));
    /// assert_eq!(mat.get_col(3), None)
    /// ```
    pub fn get_col(&self, index: usize) -> Option<Vec<T>> {
        if index >= self.width {
            None
        } else {
            let mut body = Vec::new();
            let sliced = self.as_slice();

            for row in 0..self.height {
                body.push(sliced[row * self.width + index]);
            }

            Some(body)
        }
    }

    /// Returns a `Vec` of all the columns
    /// ```
    /// extern crate mtrs;
    /// use mtrs::matrix;
    ///
    /// let mat = matrix![i32; (3, 2); 1, 2; 3, 4; 5, 6];
    ///
    /// assert_eq!(mat.cols(), vec![vec![1, 3, 5], vec![2, 4, 6]]);
    /// ```
    pub fn cols(&self) -> Vec<Vec<T>> {
        let mut body = Vec::new();

        for i in 0..self.width {
            // We can call `unwrap` here as it is guaranteed to be within bounds
            body.push(self.get_col(i).unwrap());
        }

        body
    }

    /// Returns an entry in the Matrix safely, that is:
    /// ```
    /// extern crate mtrs;
    /// use mtrs::matrix;
    ///
    /// let mat = matrix![i32; (2, 2); 1, 2; 3, 4];
    ///
    /// assert_eq!(mat.get((0, 1)), Some(&2));
    /// ```
    pub fn get<S: Size>(&self, loc: S) -> Option<&T> {
        let (h, w) = loc.dim();
        self.data.get(h * self.width + w)
    }

    /// Sets an entry in the Matrix
    /// ```
    /// #[macro_use] extern crate mtrs;
    /// use mtrs::Matrix;
    ///
    /// let mut mat = matrix![(2, 3); 1, 2, 3; 4, 5, 6];
    /// mat.set(1, 13);
    /// assert_eq!(mat.as_slice(), &[1, 2, 3, 4, 13, 6]);
    /// ```
    pub fn set<S: Size>(&mut self, loc: S, val: T) -> io::Result<()> {
        let (h, w) = loc.dim();
        if h >= self.height || w >= self.width {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid bounds"))
        } else {
            self.data[h * self.width + w] = val;
            Ok(())
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::Matrix;

    #[test]
    fn test_cols() {
        let matrix1: Matrix<i32> = Matrix::identity(1);
        let matrix2: Matrix<i32> = Matrix::identity(2);
        let matrix3: Matrix<i32> = Matrix::identity(3);

        assert_eq!(matrix1.get_col(0), Some(vec![1]));
        assert_eq!(matrix2.get_col(0), Some(vec![1, 0]));
        assert_eq!(matrix2.get_col(1), Some(vec![0, 1]));
        assert_eq!(matrix3.get_col(0), Some(vec![1, 0, 0]));
        assert_eq!(matrix3.get_col(1), Some(vec![0, 1, 0]));
        assert_eq!(matrix3.get_col(2), Some(vec![0, 0, 1]));

        assert_eq!(matrix1.cols(), vec![vec![1]]);
        assert_eq!(matrix2.cols(), vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(
            matrix3.cols(),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]
        );
    }

    #[test]
    fn test_empty_matrix() {
        let matrix0: Matrix<i32> = Matrix::zeros(0);

        assert_eq!(matrix0.size(), (0, 0));
        assert_eq!(matrix0.get_col(0), None);
        assert_eq!(matrix0.as_slice(), &[]);
        assert_eq!(matrix0.scalar_add(0), matrix0);
    }

    #[test]
    fn test_erase() {
        let mut matrix: Matrix<i32> = Matrix::ones(2);
        unsafe {
            matrix.erase();
        }

        assert_eq!(matrix, Matrix::<i32>::zeros(2));
    }
}
