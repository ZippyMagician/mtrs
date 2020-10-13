#[macro_use]
extern crate mtrs;

use mtrs::Matrix;

#[test]
fn test_addition() {
    let m1 = matrix![(3, 3); 3, 4, 7; 1, 2, 6; 9, 5, 7;];
    let m2 = Matrix::identity(3);

    assert_eq!(m1 + m2, matrix![(3, 3); 4, 4, 7; 1, 3, 6; 9, 5, 8;]);
}

#[test]
fn test_subtraction() {
    let m1 = matrix![(3, 3); 3, 4, 7; 1, 2, 6; 9, 5, 7;];
    let m2 = Matrix::identity(3);

    assert_eq!(m1 - m2, matrix![(3, 3); 2, 4, 7; 1, 1, 6; 9, 5, 6;]);
}

#[test]
fn test_display() {
    let matrix = matrix![(2, 2); 1, 2; 3, 4];

    assert_eq!(format!("{}", matrix), "1 2 \n3 4 \n".to_string());
}

#[test]
fn test_transpose() {
    let mut matrix = matrix![(2, 2); 1, 2; 3, 4];
    let mut matrix2 = matrix![(3, 3); 3, 4, 7; 1, 2, 6; 9, 5, 7];

    matrix.transpose();
    matrix2.transpose();

    assert_eq!(matrix, matrix![(2, 2); 1, 3; 2, 4]);
    assert_eq!(matrix2, matrix![(3, 3); 3, 1, 9; 4, 2, 5; 7, 6, 7]);
}

#[test]
fn test_multiplication() {
    let matrix = matrix![(2, 3); 2, 3, 4; 1, 0, 0];
    let matrix2 = matrix![(3, 2); 0, 1000; 1, 100; 0, 10];

    assert_eq!(matrix * matrix2, matrix![(2, 2); 3, 2340; 0, 1000]);
}

#[test]
fn test_scalars() {
    let norm_matrix = matrix![(2, 2); 1, 2; 3, 4];
    let floating_matrix = matrix![f32; (2, 2); 1, 2; 3, 4];

    assert_eq!(norm_matrix.scalar_add(3), matrix![(2, 2); 4, 5; 6, 7]);
    assert_eq!(norm_matrix.scalar_sub(1), matrix![(2, 2); 0, 1; 2, 3]);
    assert_eq!(norm_matrix.scalar_mul(2), matrix![(2, 2); 2, 4; 6, 8]);
    assert_eq!(
        floating_matrix.scalar_div(2.0),
        matrix![f32; (2, 2); 0.5, 1; 1.5, 2]
    );
}

#[test]
fn test_determinant() {
    let matrix2x2 = matrix![(2, 2); 1, 2; 3, 4];
    let matrix3x3 = matrix![(3, 3); 1, 2, 3; 4, 5, 6; 7, 8, 9];
    let matrix4x4 = matrix![(4, 4); 1, 0, 2, -1; 3, 0, 0, 5; 2, 1, 4, -3; 1, 0, 5, 0];

    assert_eq!(matrix2x2.determinant(), Some(-2));
    assert_eq!(matrix3x3.determinant(), Some(0));
    assert_eq!(matrix4x4.determinant(), Some(30));
}

#[test]
fn test_inverse() {
    let matrix = matrix![f32; (2, 2); -1, 1.5; 1, -1];

    assert_eq!(matrix.determinant(), Some(-0.5));
    assert_eq!(
        matrix.inverse().expect("Could not take inverse"),
        matrix![f32; (2, 2); 2, -3; -2, 2]
    );
}
