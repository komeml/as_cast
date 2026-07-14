use crate::utility::simple_as;

/// 損失無しのキャスト(拡大・同幅同符号など)
macro_rules! saturating_cast_lossless {
    ($v:ident, $src:ty, $dst:ty) => {{ simple_as!($v, $dst) }};
}

/// TryFromを活用した整数→整数へのキャスト用
macro_rules! saturating_cast_clamp_both {
    ($v:ident, $src:ty, $dst:ty) => {{
        // 符号なし型では`$v < 0`が常にfalseとなりunused_comparisons警告が出るため抑制する
        #[allow(unused_comparisons)]
        let clamped = if let Ok(v) = <$dst>::try_from($v) {
            v
        } else if $v < 0 {
            <$dst>::MIN
        } else {
            <$dst>::MAX
        };
        clamped
    }};
}

/// f64→f32へのキャストで飽和させるための処理
macro_rules! saturating_cast_float_to_float {
    ($v:ident, $src:ty, $dst:ty) => {{
        let y = $v as $dst;
        if y.is_infinite() && $v.is_finite() {
            if $v.is_sign_positive() {
                <$dst>::MAX
            } else {
                <$dst>::MIN
            }
        } else {
            y
        }
    }};
}

macro_rules! saturating_cast_u128_to_f32 {
    ($v:ident, $src:ty, $dst:ty) => {{
        let y = $v as $dst;
        if y.is_infinite() { <$dst>::MAX } else { y }
    }};
}

macro_rules! trait_saturating_cast {
    ( $($trait:ident :: $method:ident -> $target:ty);+ $(;)?) => {
        $(
            pub trait $trait {
                fn $method(self) -> $target;
            }
        )+
    };
}

// macroには、`saturating_cast_**`マクロを指定する
macro_rules! impl_saturating_cast_macro {
    ($trait:ident :: $method:ident -> $target:ty, $macro:ident; $($src:tt),+ $(,)?) => {
        $(
            impl $trait for $src {
                #[inline]
                fn $method(self) -> $target {
                    $macro!(self, $src, $target)
                }
            }
        )+
    };
}

trait_saturating_cast!(
    SaturatingCastU8::saturating_cast_u8 -> u8;
    SaturatingCastU16::saturating_cast_u16 -> u16;
    SaturatingCastU32::saturating_cast_u32 -> u32;
    SaturatingCastU64::saturating_cast_u64 -> u64;
    SaturatingCastU128::saturating_cast_u128 -> u128;
    SaturatingCastUsize::saturating_cast_usize -> usize;
    SaturatingCastI8::saturating_cast_i8 -> i8;
    SaturatingCastI16::saturating_cast_i16 -> i16;
    SaturatingCastI32::saturating_cast_i32 -> i32;
    SaturatingCastI64::saturating_cast_i64 -> i64;
    SaturatingCastI128::saturating_cast_i128 -> i128;
    SaturatingCastIsize::saturating_cast_isize -> isize;
    SaturatingCastF32::saturating_cast_f32 -> f32;
    SaturatingCastF64::saturating_cast_f64 -> f64;
);

// 符号なし整数へ
// 拡大キャストは損失が発生しない(usizeはstdのFrom実装と同様に16bit幅の可能性を考慮する)
impl_saturating_cast_macro!(SaturatingCastU16::saturating_cast_u16 -> u16, saturating_cast_lossless; u8);
impl_saturating_cast_macro!(SaturatingCastU32::saturating_cast_u32 -> u32, saturating_cast_lossless; u8, u16);
impl_saturating_cast_macro!(SaturatingCastU64::saturating_cast_u64 -> u64, saturating_cast_lossless; u8, u16, u32);
impl_saturating_cast_macro!(SaturatingCastU128::saturating_cast_u128 -> u128, saturating_cast_lossless; u8, u16, u32, u64);
impl_saturating_cast_macro!(SaturatingCastUsize::saturating_cast_usize -> usize, saturating_cast_lossless; u8, u16);

impl_saturating_cast_macro!(SaturatingCastU8::saturating_cast_u8 -> u8, saturating_cast_clamp_both; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastU16::saturating_cast_u16 -> u16, saturating_cast_clamp_both; u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastU32::saturating_cast_u32 -> u32, saturating_cast_clamp_both; u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastU64::saturating_cast_u64 -> u64, saturating_cast_clamp_both; u128, usize, i8, i16, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastU128::saturating_cast_u128 -> u128, saturating_cast_clamp_both; usize, i8, i16, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastUsize::saturating_cast_usize -> usize, saturating_cast_clamp_both; u32, u64, u128, i8, i16, i32, i64, i128, isize);

// 浮動小数点→整数は`as`自体が飽和キャスト(NaNは0)を行う
impl_saturating_cast_macro!(SaturatingCastU8::saturating_cast_u8 -> u8, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastU16::saturating_cast_u16 -> u16, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastU32::saturating_cast_u32 -> u32, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastU64::saturating_cast_u64 -> u64, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastU128::saturating_cast_u128 -> u128, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastUsize::saturating_cast_usize -> usize, saturating_cast_lossless; f32, f64);

// 符号あり整数へ
impl_saturating_cast_macro!(SaturatingCastI16::saturating_cast_i16 -> i16, saturating_cast_lossless; u8, i8);
impl_saturating_cast_macro!(SaturatingCastI32::saturating_cast_i32 -> i32, saturating_cast_lossless; u8, u16, i8, i16);
impl_saturating_cast_macro!(SaturatingCastI64::saturating_cast_i64 -> i64, saturating_cast_lossless; u8, u16, u32, i8, i16, i32);
impl_saturating_cast_macro!(SaturatingCastI128::saturating_cast_i128 -> i128, saturating_cast_lossless; u8, u16, u32, u64, i8, i16, i32, i64);
impl_saturating_cast_macro!(SaturatingCastIsize::saturating_cast_isize -> isize, saturating_cast_lossless; u8, i8, i16);

impl_saturating_cast_macro!(SaturatingCastI8::saturating_cast_i8 -> i8, saturating_cast_clamp_both; u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastI16::saturating_cast_i16 -> i16, saturating_cast_clamp_both; u16, u32, u64, u128, usize, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastI32::saturating_cast_i32 -> i32, saturating_cast_clamp_both; u32, u64, u128, usize, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastI64::saturating_cast_i64 -> i64, saturating_cast_clamp_both; u64, u128, usize, i128, isize);
impl_saturating_cast_macro!(SaturatingCastI128::saturating_cast_i128 -> i128, saturating_cast_clamp_both; u128, usize, isize);
impl_saturating_cast_macro!(SaturatingCastIsize::saturating_cast_isize -> isize, saturating_cast_clamp_both; u16, u32, u64, u128, usize, i32, i64, i128);

// 浮動小数点→整数は`as`自体が飽和キャスト(NaNは0)を行う
impl_saturating_cast_macro!(SaturatingCastI8::saturating_cast_i8 -> i8, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastI16::saturating_cast_i16 -> i16, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastI32::saturating_cast_i32 -> i32, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastI64::saturating_cast_i64 -> i64, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastI128::saturating_cast_i128 -> i128, saturating_cast_lossless; f32, f64);
impl_saturating_cast_macro!(SaturatingCastIsize::saturating_cast_isize -> isize, saturating_cast_lossless; f32, f64);

// 浮動小数点へ
impl_saturating_cast_macro!(SaturatingCastF32::saturating_cast_f32 -> f32, saturating_cast_lossless; u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize);
impl_saturating_cast_macro!(SaturatingCastF64::saturating_cast_f64 -> f64, saturating_cast_lossless; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32);

// u128のみf32の範囲を超える値を持つため飽和処理を行う
impl_saturating_cast_macro!(SaturatingCastF32::saturating_cast_f32 -> f32, saturating_cast_u128_to_f32; u128);
impl_saturating_cast_macro!(SaturatingCastF32::saturating_cast_f32 -> f32, saturating_cast_float_to_float; f64);
