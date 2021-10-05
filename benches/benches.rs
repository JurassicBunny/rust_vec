use criterion::{criterion_group, criterion_main, Criterion};
use vectored::Vector3D;

fn benches(c: &mut Criterion) {
    let vector = Vector3D::new(1.0, 2.0, 3.0);
    c.bench_function("Vector3D::new()", |b| {
        b.iter(|| Vector3D::new(1.0, 2.0, 3.0))
    });
    c.bench_function("Vector3D::norm()", |b| b.iter(|| vector.norm()));
    c.bench_function("Vector3D::sqr_norm()", |b| b.iter(|| vector.sqr_norm()));
    c.bench_function("Vector3D::normalize()", |b| b.iter(|| vector.normalize()));
}

criterion_group!(my_bench, benches);
criterion_main!(my_bench);
