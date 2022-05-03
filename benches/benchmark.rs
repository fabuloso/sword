use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("prova", |b| {
        b.to_async(&rt).iter(|| sword::read_uk_sanctions_list());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);