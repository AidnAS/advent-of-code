use std::time::Duration;

use aoc::cli::DAYS;
use aoc::{cli, solver, utils};

const MIN_WARMUP_ITERATIONS: u32 = 10;
const MIN_MEASURE_ITERATIONS: u32 = 10;
const WARMUP_DURATION: Duration = Duration::new(1, 0);
const MEASURE_DURATION: Duration = Duration::new(1, 0);

fn main() {
    println!("Day{:8}Part 1{:10}Part 2", "", "");
    println!("-----------------------------------");
    let mut total_duration = Duration::ZERO;
    for day in 1..=DAYS {
        let mut durations = [Duration::ZERO, Duration::ZERO];
        for part in 1..=2 {
            let function = cli::get_function(day, part);

            let duration_sample = solver::solve(day, part, function).duration;
            let warmup_iterations = MIN_WARMUP_ITERATIONS
                .max((WARMUP_DURATION.as_nanos() / duration_sample.as_nanos()) as u32);

            let mut warmup_time = Duration::ZERO;
            for _ in 0..warmup_iterations {
                warmup_time += solver::solve(day, part, function).duration;
            }

            let average_warmup_time = warmup_time / warmup_iterations;
            let measure_iterations = MIN_MEASURE_ITERATIONS
                .max((MEASURE_DURATION.as_nanos() / average_warmup_time.as_nanos()) as u32);

            let mut measurement_time = Duration::ZERO;
            for _ in 0..measure_iterations {
                measurement_time += solver::solve(day, part, function).duration;
            }

            let average_measurement_time = measurement_time / measure_iterations;

            durations[part as usize - 1] = average_measurement_time;
            total_duration += average_measurement_time;
        }

        println!(
            "{:02}{:6}{:>10}{:6}{:>10}",
            day,
            "",
            utils::format_duration(&durations[0]),
            "",
            utils::format_duration(&durations[1]),
        );
    }
    println!("-----------------------------------");
    println!("Total: {}", utils::format_duration(&total_duration));
}
