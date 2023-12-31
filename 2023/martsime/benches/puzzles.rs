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
    group.bench_function("day04part01", |b| b.iter(aoc::day04::part01));
    group.bench_function("day04part02", |b| b.iter(aoc::day04::part02));
    group.bench_function("day05part01", |b| b.iter(aoc::day05::part01));
    group.bench_function("day05part02", |b| b.iter(aoc::day05::part02));
    group.bench_function("day06part01", |b| b.iter(aoc::day06::part01));
    group.bench_function("day06part02", |b| b.iter(aoc::day06::part02));
    group.bench_function("day07part01", |b| b.iter(aoc::day07::part01));
    group.bench_function("day07part02", |b| b.iter(aoc::day07::part02));
    group.bench_function("day08part01", |b| b.iter(aoc::day08::part01));
    group.bench_function("day08part02", |b| b.iter(aoc::day08::part02));
    group.bench_function("day09part01", |b| b.iter(aoc::day09::part01));
    group.bench_function("day09part02", |b| b.iter(aoc::day09::part02));
    group.bench_function("day10part01", |b| b.iter(aoc::day10::part01));
    group.bench_function("day10part02", |b| b.iter(aoc::day10::part02));
    group.bench_function("day11part01", |b| b.iter(aoc::day11::part01));
    group.bench_function("day11part02", |b| b.iter(aoc::day11::part02));
    group.bench_function("day12part01", |b| b.iter(aoc::day12::part01));
    group.bench_function("day12part02", |b| b.iter(aoc::day12::part02));
    group.bench_function("day13part01", |b| b.iter(aoc::day13::part01));
    group.bench_function("day13part02", |b| b.iter(aoc::day13::part02));
    group.bench_function("day14part01", |b| b.iter(aoc::day14::part01));
    group.bench_function("day14part02", |b| b.iter(aoc::day14::part02));
    group.bench_function("day15part01", |b| b.iter(aoc::day15::part01));
    group.bench_function("day15part02", |b| b.iter(aoc::day15::part02));
    group.bench_function("day16part01", |b| b.iter(aoc::day16::part01));
    group.bench_function("day16part02", |b| b.iter(aoc::day16::part02));
    group.bench_function("day17part01", |b| b.iter(aoc::day17::part01));
    group.bench_function("day17part02", |b| b.iter(aoc::day17::part02));
    group.bench_function("day18part01", |b| b.iter(aoc::day18::part01));
    group.bench_function("day18part02", |b| b.iter(aoc::day18::part02));
    group.finish()
}

criterion_group!(benches, bench_puzzles);
criterion_main!(benches);
