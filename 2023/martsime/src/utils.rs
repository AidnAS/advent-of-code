use std::str::{from_utf8, FromStr};

#[inline(always)]
pub fn parse_number_from_bytes<T: FromStr>(bytes: &[u8]) -> T {
    from_utf8(bytes).unwrap().parse::<T>().ok().unwrap()
}
