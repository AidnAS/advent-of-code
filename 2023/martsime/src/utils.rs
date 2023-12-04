use std::str::{from_utf8, FromStr};
use std::time::Duration;

#[inline(always)]
pub fn parse_number_from_bytes<T: FromStr>(bytes: &[u8]) -> T {
    from_utf8(bytes).unwrap().parse::<T>().ok().unwrap()
}

pub fn format_duration(duration: &Duration) -> String {
    let nanos = duration.as_nanos() as f64;
    if nanos < 1_000.0 {
        format!("{:7.3}ns", nanos)
    } else if nanos < 1_000_000.0 {
        format!("{:7.3}μs", nanos / 1_000.0)
    } else if nanos < 1_000_000_000.0 {
        format!("{:7.3}ms", nanos / 1_000_000.0)
    } else {
        format!("{:7.3}s ", nanos / 1_000_000_000.0)
    }
}
