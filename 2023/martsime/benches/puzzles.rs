use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_puzzles(c: &mut Criterion) {
    let mut group = c.benchmark_group("aoc");
    group.warm_up_time(Duration::from_secs_f64(1.0));
    group.measurement_time(Duration::from_secs_f64(1.0));
    group.bench_function("day01part01", |b| b.iter(aoc::day01::part01));
    group.bench_function("day01part02", |b| b.iter(aoc::day01::part02));
    group.bench_function("day02part01", |b| b.iter(aoc::day02::part01));
    group.bench_function("day02part02", |b| b.iter(aoc::day02::part02));
    group.bench_function("day03part01", |b| b.iter(aoc::day03::part01));
    group.bench_function("day03part02", |b| b.iter(aoc::day03::part02));
    group.finish()
}

criterion_group!(benches, bench_puzzles);
criterion_main!(benches);
