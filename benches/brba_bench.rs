use std::hint::{black_box};
use criterion::{criterion_group, criterion_main, Criterion};
use brba_text::process_text;

fn benchmark_brba(c: &mut Criterion) {
    // We use Moby Dick content for a heavy workload
    let input = std::fs::read_to_string("inputs/moby_dick.txt").unwrap();

    c.bench_function("process_text_large", |b| {
        b.iter(|| process_text(black_box(&input.as_bytes())))
    });
}

criterion_group!(benches, benchmark_brba);
criterion_main!(benches);