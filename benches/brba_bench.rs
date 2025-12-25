use std::hint::{black_box};
use criterion::{criterion_group, criterion_main, Criterion};
use brba_text::process_text;


fn benchmark_random(c: &mut Criterion) {
    // Random data
    let input = std::fs::read_to_string("inputs/random.txt").unwrap();

    c.bench_function("process_text_random", |b| {
        b.iter_with_large_drop(|| process_text(black_box(&input.as_bytes())))
    });
}

criterion_group!(benches, benchmark_random);
criterion_main!(benches);