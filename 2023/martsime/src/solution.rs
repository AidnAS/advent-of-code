use std::fmt::{Display, Formatter, Result};

pub enum Solution {
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    String(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::U32(value) => write!(f, "{}", value),
            Self::I32(value) => write!(f, "{}", value),
            Self::U64(value) => write!(f, "{}", value),
            Self::I64(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}

impl From<u32> for Solution {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<i32> for Solution {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<u64> for Solution {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<i64> for Solution {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<String> for Solution {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
