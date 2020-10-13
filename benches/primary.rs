extern crate mtrs;

use mtrs::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench_fn {
    ($val:expr, $id:expr, $fn:expr) => {
        $val.bench_function($id, |b| b.iter(|| black_box($fn)))
    };
}

fn test_transpose() {
    let mut matrix = matrix![i32; (4, 4); 1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, 16];

    matrix.transpose();
}

fn test_ops() {
    let matrix = matrix![i32; (4, 4); 1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, 16];

    let _a = matrix.scalar_add(13);
    let _b = matrix.scalar_sub(3);
    let _c = matrix.scalar_div(2);
    let _d = matrix.scalar_mul(5);
}

fn test_determinant() {
    let matrix = matrix![i32; (4, 4); 1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, 16];

    let _a = matrix.determinant();
}

fn benchmark(c: &mut Criterion) {
    bench_fn!(c, "transpose", test_transpose());
    bench_fn!(c, "scalar_ops", test_ops());
    bench_fn!(c, "determinant", test_determinant());
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
