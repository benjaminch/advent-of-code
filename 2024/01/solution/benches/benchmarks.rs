use aoc_2024_solution_01::{get_lists_from_str, get_similarity, get_total_distance};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_get_lists_from_str(c: &mut Criterion) {
    c.bench_function("get_lists_from_str", |b| {
        b.iter(|| get_lists_from_str("1   2\n3   4\n"));
    });
}

fn bench_get_total_distance(c: &mut Criterion) {
    c.bench_function("get_total_distance", |b| {
        b.iter(|| {
            get_total_distance(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]).expect("should not fail")
        });
    });
}

fn bench_get_similarity(c: &mut Criterion) {
    c.bench_function("get_similarity", |b| {
        b.iter(|| {
            get_similarity(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]).expect("should not fail")
        });
    });
}

criterion_group!(
    benches,
    bench_get_lists_from_str,
    bench_get_total_distance,
    bench_get_similarity
);
criterion_main!(benches);
