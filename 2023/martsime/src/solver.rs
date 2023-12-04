use std::fmt::{Display, Formatter, Result};
use std::hint::black_box;
use std::time::Duration;

use crate::solution::Solution;
use crate::utils;

pub struct PartResult {
    pub day: u8,
    pub part: u8,
    pub result: Solution,
    pub duration: Duration,
}

impl Display for PartResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[{:>9}] Day {:02} Part {:02}: {}",
            utils::format_duration(&self.duration),
            self.day,
            self.part,
            self.result
        )
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
