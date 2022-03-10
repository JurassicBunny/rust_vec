use criterion::{criterion_group, criterion_main, Criterion};
use vectored::{Acceleration, Vectored};

fn benches(c: &mut Criterion) {
    let vector = Acceleration::new(1.0, 2.0, 3.0);
    let vector_2 = Acceleration::new(1.0, 2.0, 3.0);
    c.bench_function("Vectored::new()", |b| {
        b.iter(|| Acceleration::new(1.0, 2.0, 3.0))
    });
    c.bench_function("Vectored::norm()", |b| b.iter(|| vector.norm()));
    c.bench_function("Vectored::sqr_norm()", |b| b.iter(|| vector.sqr_norm()));
    c.bench_function("Vectored::normalize()", |b| b.iter(|| vector.normalize()));
    c.bench_function("Vectored::addition()", |b| b.iter(|| vector + vector_2));
    c.bench_function("Vectored::subtraction()", |b| b.iter(|| vector - vector_2));
}

criterion_group!(my_bench, benches);
criterion_main!(my_bench);
