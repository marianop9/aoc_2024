use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
#[allow(dead_code)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Solution::I8(x) => x.fmt(f),
            Solution::I16(x) => x.fmt(f),
            Solution::I32(x) => x.fmt(f),
            Solution::I64(x) => x.fmt(f),
            Solution::I128(x) => x.fmt(f),
            Solution::Isize(x) => x.fmt(f),
            Solution::U8(x) => x.fmt(f),
            Solution::U16(x) => x.fmt(f),
            Solution::U32(x) => x.fmt(f),
            Solution::U64(x) => x.fmt(f),
            Solution::U128(x) => x.fmt(f),
            Solution::Usize(x) => x.fmt(f),
            Solution::Str(x) => x.fmt(f),
        }
    }
}