use std::fmt::{Display, Formatter, Result};
use std::hint::black_box;
use std::time::Duration;

use crate::solution::Solution;

pub struct PartResult {
    pub day: u8,
    pub part: u8,
    pub result: Solution,
    pub duration: Duration,
}

impl Display for PartResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "==================")?;
        writeln!(f, "Day {:02} Part {:02}", self.day, self.part,)?;
        let nanos = self.duration.as_nanos() as f64;
        if nanos < 1_000.0 {
            writeln!(f, "Runtime: {:7.3}ns", nanos)?;
        } else if nanos < 1_000_000.0 {
            writeln!(f, "Runtime: {:7.3}μs", nanos / 1_000.0)?;
        } else if nanos < 1_000_000_000.0 {
            writeln!(f, "Runtime: {:7.3}ms", nanos / 1_000_000.0)?;
        } else {
            writeln!(f, "Runtime: {:7.3}s ", nanos / 1_000_000_000.0)?;
        }
        writeln!(f, "Result: {}", self.result)
    }
}

#[inline(always)]
pub fn solve(day: u8, part: u8, function: fn() -> Solution) -> PartResult {
    let start_time = std::time::Instant::now();
    let result = black_box(function());
    let end_time = std::time::Instant::now();
    let duration = end_time - start_time;

    PartResult {
        day,
        part,
        result,
        duration,
    }
}
