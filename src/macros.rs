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
        mtrs::Matrix::from_slice(($height, $length), &[$($($val,)*)*])
    };

    // matrix![f32; (2, 2); 1, 2; 3, 4.2]
    ($type:ty; ($height:expr,$length:expr); $($($val:expr),*);*) => {
        mtrs::Matrix::<$type>::from_slice(($height, $length), &[$($($val as $type,)*)*])
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
