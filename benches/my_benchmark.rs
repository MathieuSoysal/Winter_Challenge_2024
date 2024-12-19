use codingame_rust_template::fib::fibonacci;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| black_box(fibonacci(20))));
    c.bench_function("fib 30", |b| b.iter(|| black_box(fibonacci(30))));
    c.bench_function("fib 40", |b| b.iter(|| black_box(fibonacci(40))));
    c.bench_function("fib 50", |b| b.iter(|| black_box(fibonacci(50))));
    c.bench_function("fib 60", |b| b.iter(|| black_box(fibonacci(60))));
    c.bench_function("fib 70", |b| b.iter(|| black_box(fibonacci(70))));
    c.bench_function("fib 80", |b| b.iter(|| black_box(fibonacci(80))));
    c.bench_function("fib 90", |b| b.iter(|| black_box(fibonacci(90))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
