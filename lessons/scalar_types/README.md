# Scalar Types

This crate presents the simplest algebraic data types that
are available in rust.

Signed integers
types: i8, i16, i32, i64, i128, isize (1 cpu word)
litterals: -10, 0, 1_000, 123i64

Unsigned integers
types: u8, u16, u32, u64, u128, usize (1 cpu word)
litterals: 0, 123, 10u16

Floating point numbers
types: f32, f64
litterals: 3.14, -10.0e20, 2f32

Strings
types: &str
litterals: "foo", "two\nlines"

Unicode scalar values
types: char (32b)
litterals: 'a', 'α', '∞'

Booleans
types: bool
litterals: true, false