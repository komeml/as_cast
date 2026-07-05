use crate::utility::{
    can_convert_int_to_float, can_convert_signed_int_to_float, can_convert_unsigned_int_to_float,
    convert_float_to_float, convert_float_to_int, convert_int_to_int, simple_as,
};

macro_rules! trait_checked_cast {
    ( $($trait:ident :: $method:ident -> $target:ty);+ $(;)?) => {
        $(
            pub trait $trait {
                fn $method(self) -> Option<$target>;
            }
        )+
    };
}

macro_rules! impl_checked_cast_int_to_float {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> Option<$target> {
                    if can_convert_int_to_float!(self, $target, $src) {
                        Some(simple_as!(self, $target))
                    } else {
                        None
                    }
                }
            }
        )+
    };
}

macro_rules! impl_checked_cast_float_to_int {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> Option<$target> {
                    convert_float_to_int!(self, $target, $src)
                }
            }
        )+
    };
}

macro_rules! impl_checked_cast_int_to_int {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> Option<$target> {
                    convert_int_to_int!(self, $target)
                }
            }
        )+
    };
}

macro_rules! impl_checked_cast_float_to_float {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> Option<$target> {
                    convert_float_to_float!(self, $target, $src)
                }
            }
        )+
    };
}

trait_checked_cast!(
    CheckedCastU8::checked_cast_u8 -> u8;
    CheckedCastU16::checked_cast_u16 -> u16;
    CheckedCastU32::checked_cast_u32 -> u32;
    CheckedCastU64::checked_cast_u64 -> u64;
    CheckedCastU128::checked_cast_u128 -> u128;
    CheckedCastUsize::checked_cast_usize -> usize;
    CheckedCastI8::checked_cast_i8 -> i8;
    CheckedCastI16::checked_cast_i16 -> i16;
    CheckedCastI32::checked_cast_i32 -> i32;
    CheckedCastI64::checked_cast_i64 -> i64;
    CheckedCastI128::checked_cast_i128 -> i128;
    CheckedCastIsize::checked_cast_isize -> isize;
    CheckedCastF32::checked_cast_f32 -> f32;
    CheckedCastF64::checked_cast_f64 -> f64;
);

// 符号なし整数へ
impl_checked_cast_int_to_int!(CheckedCastU8::checked_cast_u8 -> u8; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastU16::checked_cast_u16 -> u16; u8, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastU32::checked_cast_u32 -> u32; u8, u16, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastU64::checked_cast_u64 -> u64; u8, u16, u32, u128, usize, i8, i16, i32, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastU128::checked_cast_u128 -> u128; u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastUsize::checked_cast_usize -> usize; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize);

impl_checked_cast_float_to_int!(CheckedCastU8::checked_cast_u8 -> u8; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastU16::checked_cast_u16 -> u16; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastU32::checked_cast_u32 -> u32; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastU64::checked_cast_u64 -> u64; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastU128::checked_cast_u128 -> u128; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastUsize::checked_cast_usize -> usize; f32, f64);

// 符号あり整数へ
impl_checked_cast_int_to_int!(CheckedCastI8::checked_cast_i8 -> i8; u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastI16::checked_cast_i16 -> i16; u8, u16, u32, u64, u128, usize, i8, i32, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastI32::checked_cast_i32 -> i32; u8, u16, u32, u64, u128, usize, i8, i16, i64, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastI64::checked_cast_i64 -> i64; u8, u16, u32, u64, u128, usize, i8, i16, i32, i128, isize);
impl_checked_cast_int_to_int!(CheckedCastI128::checked_cast_i128 -> i128; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize);
impl_checked_cast_int_to_int!(CheckedCastIsize::checked_cast_isize -> isize; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128);

impl_checked_cast_float_to_int!(CheckedCastI8::checked_cast_i8 -> i8; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastI16::checked_cast_i16 -> i16; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastI32::checked_cast_i32 -> i32; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastI64::checked_cast_i64 -> i64; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastI128::checked_cast_i128 -> i128; f32, f64);
impl_checked_cast_float_to_int!(CheckedCastIsize::checked_cast_isize -> isize; f32, f64);

// 浮動小数点へ
impl_checked_cast_int_to_float!(CheckedCastF32::checked_cast_f32 -> f32; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_checked_cast_int_to_float!(CheckedCastF64::checked_cast_f64 -> f64; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl_checked_cast_float_to_float!(CheckedCastF32::checked_cast_f32 -> f32; f64);
impl_checked_cast_float_to_float!(CheckedCastF64::checked_cast_f64 -> f64; f32);
