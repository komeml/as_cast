# as_cast
[![crates.io](https://img.shields.io/crates/v/as_cast.svg)](https://crates.io/crates/as_cast)
[![docs](https://docs.rs/as_cast/badge.svg)](https://docs.rs/as_cast)
[![Rust](https://img.shields.io/badge/rust-1.85.0%2B-blue.svg?maxAge=3600)](https://github.com/komeml/as_cast)

[日本語](https://github.com/komeml/as_cast/blob/main/README-jp.md)

## Overview
Rust's built-in `self as T` casting has a readability problem: as casts pile up, the code becomes harder to follow.

`as_cast` is a crate that solves this by providing casts in **method form**, such as `cast_i32()` and `cast_f64()`.

It also provides lossless-aware variants like `checked_cast_i32()` and `checked_cast_f64()` that account for loss during casting.

## Features
Each feature can be enabled individually via feature flags (`cast` / `checked-cast` / `saturating-cast`). All of them are enabled by default.

### Cast*

A set of traits that perform the standard `as` cast in method form. From `cast_u8()` to `cast_f64()`, conversions are supported between all numeric types (`u8`–`u128`, `usize`, `i8`–`i128`, `isize`, `f32`, `f64`).

The behavior is exactly the same as `as`, so **any loss goes undetected**. If you want to detect loss, use `CheckedCast*` instead.

```rust
use as_cast::cast::{CastU8, CastF64};

let n: i32 = 300;

assert_eq!(n.cast_u8(), 44); // truncated, same behavior as `as`
assert_eq!(n.cast_f64(), 300.0);
```

### CheckedCast*

A set of traits that can detect loss during conversion. `checked_cast_*()` returns an `Option<T>`, yielding `Some` **only when the conversion is lossless**.

`None` is returned in the following cases:

- The value is out of range for the target type (e.g. `300i32` → `u8`)
- A float-to-integer conversion would lose the fractional part (e.g. `1.5f64` → `u8`)
- An integer-to-float conversion would exceed the mantissa precision (e.g. `16_777_217i32` → `f32`)
- Casting `NaN`

```rust
use as_cast::checked_cast::CheckedCastU8;

let n: i32 = 200;
assert_eq!(n.checked_cast_u8(), Some(200));

let m: i32 = 300;
assert_eq!(m.checked_cast_u8(), None); // out of range for u8

let f: f64 = 1.5;
assert_eq!(f.checked_cast_u8(), None); // fractional part would be lost
```

### SaturatingCast*

A set of traits that behave just like `Cast*`, except that out-of-range values are clamped (saturated) to the target type's minimum or maximum.

Values saturate as follows:

- A value out of range for the target type → the type's min/max (e.g. `300i32` → `u8` yields `255`)
- A conversion to a float type that would overflow to infinity → `f32::MIN`/`f32::MAX` (e.g. `f64::MAX` → `f32` yields `f32::MAX`)
- `±∞` → the type's min/max for integer targets; kept as `±∞` for float targets
- `NaN` → `0` for integer targets; kept as `NaN` for float targets

Note that, just like `as`, truncation of the fractional part and loss of mantissa precision go undetected. If you want to detect loss, use `CheckedCast*` instead.

```rust
use as_cast::saturating_cast::{SaturatingCastU8, SaturatingCastF32};

let n: i32 = 300;
assert_eq!(n.saturating_cast_u8(), 255); // saturates to u8::MAX

let m: i32 = -1;
assert_eq!(m.saturating_cast_u8(), 0); // saturates to u8::MIN

let f: f64 = f64::MAX;
assert_eq!(f.saturating_cast_f32(), f32::MAX); // saturates to f32::MAX instead of infinity
```

## Roadmap
- [x] `Cast*` : casts with the same behavior as `as`
- [x] `CheckedCast*` : returns `None` when loss would occur
- [x] `SaturatingCast*` : clamps out-of-range values to the type's min/max
- [ ] `StrictCast*` : panics when loss would occur
