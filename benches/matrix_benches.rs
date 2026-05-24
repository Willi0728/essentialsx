use criterion::{criterion_group, criterion_main, Criterion};
use essentialsx::math::Matrix;
use core::hint::black_box;

fn bench_matrix_math(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix Math Operations");

    let m4_a = Matrix::<4, 4>::identity();
    let m4_b = Matrix::<4, 4>::identity();

    group.bench_function("mul_inline_4x4", |b| {
        b.iter(|| black_box(&m4_a).mul_inline(black_box(&m4_b)))
    });

    group.bench_function("add_operator_4x4", |b| {
        b.iter(|| black_box(m4_a.clone()) + black_box(m4_b.clone()))
    });

    let m32_a = Matrix::<32, 32>::identity();
    let m32_b = Matrix::<32, 32>::identity();

    group.bench_function("mul_inline_32x32", |b| {
        b.iter(|| black_box(&m32_a).mul_inline(black_box(&m32_b)))
    });

    group.finish();
}

fn bench_linear_algebra(c: &mut Criterion) {
    let mut group = c.benchmark_group("Linear Algebra Solvers");

    let m2 = Matrix::<2, 2>([[1.5, 2.0], [3.0, 4.5]]);
    group.bench_function("determinant_specialized_2x2", |b| {
        b.iter(|| black_box(&m2).determinant_2x2())
    });

    let m8 = Matrix::<8, 8>::identity();
    group.bench_function("to_upper_triangular_8x8", |b| {
        b.iter(|| black_box(&m8).to_upper_triangular())
    });

    group.bench_function("determinant_generic_8x8", |b| {
        b.iter(|| black_box(&m8).determinant())
    });

    group.finish();
}

fn bench_matrix_mutations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix Mutations & Resizing");

    let m8 = Matrix::<8, 8>::identity();
    let mut m8_mut = Matrix::<8, 8>::identity();

    group.bench_function("transpose_inline_8x8", |b| {
        b.iter(|| black_box(&m8).transpose_inline())
    });

    group.bench_function("scale_mut_in_place_8x8", |b| {
        b.iter(|| black_box(&mut m8_mut).scale_mut(black_box(2.5)))
    });

    group.bench_function("pop_row_8x8", |b| {
        b.iter(|| black_box(&m8).pop_row::<7>(black_box(3)))
    });

    group.bench_function("pop_col_8x8", |b| {
        b.iter(|| black_box(&m8).pop_col::<7>(black_box(3)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_matrix_math,
    bench_linear_algebra,
    bench_matrix_mutations
);
criterion_main!(benches);
