macro_rules! impl_checked_cast_float {
    ($trait:ident, $method:ident, $target:ty, $src:ty) => {
        impl $trait for $src {
            #[inline]
            fn $method(self) -> Option<$target> {
                if self.is_nan() {
                    return None;
                }

                let cast = self as $target;
                if (cast as $src) == self {
                    Some(cast)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! impl_checked_cast_int {
    ($trait:ident, $method:ident, $target:ty, $src:ty) => {
        impl $trait for $src {
            #[inline]
            fn $method(self) -> Option<$target> {
                let cast = self as $target;
                if (cast as $src) == self {
                    Some(cast)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! impl_checked_cast {
    ($trait:ident, $method:ident, $target:ty, f32) => {
        impl_checked_cast_float!($trait, $method, $target, f32);
    };
    ($trait:ident, $method:ident, $target:ty, f64) => {
        impl_checked_cast_float!($trait, $method, $target, f64);
    };

    // 浮動小数点だけ専用処理で分岐済みのため整数は汎用として扱う
    ($trait:ident, $method:ident, $target:ty, $src:ty) => {
        impl_checked_cast_int!($trait, $method, $target, $src);
    };
}

macro_rules! trait_checked_cast {
    ($trait:ident :: $method:ident -> $target:ty; $($src:tt),+ $(,)?) => {
        pub trait $trait {
            fn $method(self) -> Option<$target>;
        }

        $(
            impl_checked_cast!($trait, $method, $target, $src);
        )+
    };
}
// 符号なし整数へ
trait_checked_cast!(CheckedCastU8::checked_cast_u8       -> u8;     u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastU16::checked_cast_u16     -> u16;    u8, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastU32::checked_cast_u32     -> u32;    u8, u16, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastU64::checked_cast_u64     -> u64;    u8, u16, u32, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastU128::checked_cast_u128   -> u128;   u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastUsize::checked_cast_usize -> usize;  u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize, f32, f64);

// 符号あり整数へ
trait_checked_cast!(CheckedCastI8::checked_cast_i8       -> i8;     u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastI16::checked_cast_i16     -> i16;    u8, u16, u32, u64, u128, usize, i8, i32, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastI32::checked_cast_i32     -> i32;    u8, u16, u32, u64, u128, usize, i8, i16, i64, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastI64::checked_cast_i64     -> i64;    u8, u16, u32, u64, u128, usize, i8, i16, i32, i128, isize, f32, f64);
trait_checked_cast!(CheckedCastI128::checked_cast_i128   -> i128;   u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, f32, f64);
trait_checked_cast!(CheckedCastIsize::checked_cast_isize -> isize;  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64);

// 浮動小数点へ
trait_checked_cast!(CheckedCastF32::checked_cast_f32    -> f32;     u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
trait_checked_cast!(CheckedCastF64::checked_cast_f64    -> f64;     u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32);
