macro_rules! cast_trait {
    ($trait:ident :: $method:ident -> $target:ty; $($src:ty),+ $(,)?) => {
        pub trait $trait {
            fn $method(self) -> $target;
        }

        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> $target {
                    self as $target
                }
            }
        )+
    };
}

// 符号なし整数へ
cast_trait!(CastU8::cast_u8       -> u8;    u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast_trait!(CastU16::cast_u16     -> u16;   u8, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast_trait!(CastU32::cast_u32     -> u32;   u8, u16, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast_trait!(CastU64::cast_u64     -> u64;   u8, u16, u32, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast_trait!(CastU128::cast_u128   -> u128;  u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize, f32, f64);
cast_trait!(CastUsize::cast_usize -> usize; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize, f32, f64);

// 符号あり整数へ
cast_trait!(CastI8::cast_i8       -> i8;    u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64);
cast_trait!(CastI16::cast_i16     -> i16;   u8, u16, u32, u64, u128, usize, i8, i32, i64, i128, isize, f32, f64);
cast_trait!(CastI32::cast_i32     -> i32;   u8, u16, u32, u64, u128, usize, i8, i16, i64, i128, isize, f32, f64);
cast_trait!(CastI64::cast_i64     -> i64;   u8, u16, u32, u64, u128, usize, i8, i16, i32, i128, isize, f32, f64);
cast_trait!(CastI128::cast_i128   -> i128;  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, f32, f64);
cast_trait!(CastIsize::cast_isize -> isize; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64);

// 浮動小数点へ
cast_trait!(CastF32::cast_f32     -> f32;   u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64);
cast_trait!(CastF64::cast_f64     -> f64;   u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32);
