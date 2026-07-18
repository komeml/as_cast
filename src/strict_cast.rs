use crate::utility::{
    can_convert_int_to_float, can_convert_signed_int_to_float, can_convert_unsigned_int_to_float,
    convert_float_to_float, convert_float_to_int, convert_int_to_int, simple_as,
};

/// 損失が発生した際のpanic処理
macro_rules! strict_cast_panic {
    ($v:ident, $target:ty) => {
        panic!(
            "strict cast failed: cannot cast {} to {} without loss",
            $v,
            stringify!($target)
        )
    };
}

macro_rules! trait_strict_cast {
    ( $($trait:ident :: $method:ident -> $target:ty);+ $(;)?) => {
        $(
            pub trait $trait {
                fn $method(self) -> $target;
            }
        )+
    };
}

macro_rules! impl_strict_cast_int_to_float {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> $target {
                    if can_convert_int_to_float!(self, $target, $src) {
                        simple_as!(self, $target)
                    } else {
                        strict_cast_panic!(self, $target)
                    }
                }
            }
        )+
    };
}

macro_rules! impl_strict_cast_float_to_int {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> $target {
                    match convert_float_to_int!(self, $target, $src) {
                        Some(v) => v,
                        None => strict_cast_panic!(self, $target),
                    }
                }
            }
        )+
    };
}

macro_rules! impl_strict_cast_int_to_int {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> $target {
                    match convert_int_to_int!(self, $target) {
                        Some(v) => v,
                        None => strict_cast_panic!(self, $target),
                    }
                }
            }
        )+
    };
}

macro_rules! impl_strict_cast_float_to_float {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> $target {
                    match convert_float_to_float!(self, $target, $src) {
                        Some(v) => v,
                        None => strict_cast_panic!(self, $target),
                    }
                }
            }
        )+
    };
}

trait_strict_cast!(
    StrictCastU8::strict_cast_u8 -> u8;
    StrictCastU16::strict_cast_u16 -> u16;
    StrictCastU32::strict_cast_u32 -> u32;
    StrictCastU64::strict_cast_u64 -> u64;
    StrictCastU128::strict_cast_u128 -> u128;
    StrictCastUsize::strict_cast_usize -> usize;
    StrictCastI8::strict_cast_i8 -> i8;
    StrictCastI16::strict_cast_i16 -> i16;
    StrictCastI32::strict_cast_i32 -> i32;
    StrictCastI64::strict_cast_i64 -> i64;
    StrictCastI128::strict_cast_i128 -> i128;
    StrictCastIsize::strict_cast_isize -> isize;
    StrictCastF32::strict_cast_f32 -> f32;
    StrictCastF64::strict_cast_f64 -> f64;
);

// 符号なし整数へ
impl_strict_cast_int_to_int!(StrictCastU8::strict_cast_u8 -> u8; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastU16::strict_cast_u16 -> u16; u8, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastU32::strict_cast_u32 -> u32; u8, u16, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastU64::strict_cast_u64 -> u64; u8, u16, u32, u128, usize, i8, i16, i32, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastU128::strict_cast_u128 -> u128; u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastUsize::strict_cast_usize -> usize; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize);

impl_strict_cast_float_to_int!(StrictCastU8::strict_cast_u8 -> u8; f32, f64);
impl_strict_cast_float_to_int!(StrictCastU16::strict_cast_u16 -> u16; f32, f64);
impl_strict_cast_float_to_int!(StrictCastU32::strict_cast_u32 -> u32; f32, f64);
impl_strict_cast_float_to_int!(StrictCastU64::strict_cast_u64 -> u64; f32, f64);
impl_strict_cast_float_to_int!(StrictCastU128::strict_cast_u128 -> u128; f32, f64);
impl_strict_cast_float_to_int!(StrictCastUsize::strict_cast_usize -> usize; f32, f64);

// 符号あり整数へ
impl_strict_cast_int_to_int!(StrictCastI8::strict_cast_i8 -> i8; u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastI16::strict_cast_i16 -> i16; u8, u16, u32, u64, u128, usize, i8, i32, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastI32::strict_cast_i32 -> i32; u8, u16, u32, u64, u128, usize, i8, i16, i64, i128, isize);
impl_strict_cast_int_to_int!(StrictCastI64::strict_cast_i64 -> i64; u8, u16, u32, u64, u128, usize, i8, i16, i32, i128, isize);
impl_strict_cast_int_to_int!(StrictCastI128::strict_cast_i128 -> i128; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize);
impl_strict_cast_int_to_int!(StrictCastIsize::strict_cast_isize -> isize; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128);

impl_strict_cast_float_to_int!(StrictCastI8::strict_cast_i8 -> i8; f32, f64);
impl_strict_cast_float_to_int!(StrictCastI16::strict_cast_i16 -> i16; f32, f64);
impl_strict_cast_float_to_int!(StrictCastI32::strict_cast_i32 -> i32; f32, f64);
impl_strict_cast_float_to_int!(StrictCastI64::strict_cast_i64 -> i64; f32, f64);
impl_strict_cast_float_to_int!(StrictCastI128::strict_cast_i128 -> i128; f32, f64);
impl_strict_cast_float_to_int!(StrictCastIsize::strict_cast_isize -> isize; f32, f64);

// 浮動小数点へ
impl_strict_cast_int_to_float!(StrictCastF32::strict_cast_f32 -> f32; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_strict_cast_int_to_float!(StrictCastF64::strict_cast_f64 -> f64; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl_strict_cast_float_to_float!(StrictCastF32::strict_cast_f32 -> f32; f64);
impl_strict_cast_float_to_float!(StrictCastF64::strict_cast_f64 -> f64; f32);
