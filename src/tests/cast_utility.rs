//! テスト共通のユーティリティ。
//!
//! 「ソース値セット」と比較用マクロを集約する。値そのものは各テストで
//! 共有できるよう、各型について MIN / 中間 / MAX を代表値として持つ。

// ========== ソース値セット ==========
//
// 型ごとの入力値セット（符号なし・符号あり・float で分ける）。
// - `cast_int` の網羅層では「変換先と同じ型を除いた」全ソースとして流し込む。
// - `cast_float` のオラクル層では各配列を走査し `cast == as` を確認する。
// - `saturating_cast` の網羅層・単調性プロパティでも同じ配列を流用する。
//
// いずれの用途でも、各配列は境界（MIN / MAX）と中間の代表値を含む。

pub(crate) const U8S: [u8; 3] = [0, 200, u8::MAX];
pub(crate) const U16S: [u16; 3] = [0, 40_000, u16::MAX];
pub(crate) const U32S: [u32; 3] = [0, 3_000_000_000, u32::MAX];
pub(crate) const U64S: [u64; 3] = [0, 12_000_000_000_000, u64::MAX];
pub(crate) const U128S: [u128; 3] = [0, 340_000_000_000_000_000_000, u128::MAX];
pub(crate) const USIZES: [usize; 3] = [0, 100_000, usize::MAX];

pub(crate) const I8S: [i8; 6] = [i8::MIN, -100, -1, 0, 100, i8::MAX];
pub(crate) const I16S: [i16; 6] = [i16::MIN, -20_000, -1, 0, 20_000, i16::MAX];
pub(crate) const I32S: [i32; 6] = [i32::MIN, -2_000_000_000, -1, 0, 2_000_000_000, i32::MAX];
pub(crate) const I64S: [i64; 6] = [i64::MIN, -9_000_000_000, -1, 0, 9_000_000_000, i64::MAX];
pub(crate) const I128S: [i128; 6] = [
    i128::MIN,
    -170_000_000_000_000_000_000,
    -1,
    0,
    170_000_000_000_000_000_000,
    i128::MAX,
];
pub(crate) const ISIZES: [isize; 6] = [isize::MIN, -100_000, -1, 0, 100_000, isize::MAX];

// float は NaN / ∞ / 範囲外も含める。整数へ飽和・NaN→0 する regime でも、
// また浮動小数点間で拡大・縮小する場合でも `cast == as` が保たれることを確認するため
// （オラクル比較は両辺に同じ `v` を渡すので、NaN でもビットパターンが一致する）。
pub(crate) const F32S: [f32; 15] = [
    0.0,
    -0.0,
    3.9,
    -3.9,
    42.5,
    -42.5,
    300.0,
    -5.0,
    1e30,
    -1e30,
    f32::MAX,
    f32::MIN,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN,
];
pub(crate) const F64S: [f64; 17] = [
    0.0,
    -0.0,
    3.9,
    -3.9,
    42.5,
    -42.5,
    300.0,
    -5.0,
    1e30,
    -1e30,
    1e300,
    -1e300,
    f64::MAX,
    f64::MIN,
    f64::INFINITY,
    f64::NEG_INFINITY,
    f64::NAN,
];

// ========== 仮数部境界の定数 ==========
//
// 浮動小数点が整数を連続で表せる上限（f32: 2^24、f64: 2^53）とその前後 ±1。
// `checked_cast` の整数 → 浮動小数点テストで精度落ちの境界を突くために使う。

#[cfg(feature = "checked-cast")]
pub(crate) const F32_MANTISSA_LIMIT: u32 = 1 << f32::MANTISSA_DIGITS;
#[cfg(feature = "checked-cast")]
pub(crate) const F32_MANTISSA_LIMIT_MINUS_1: u32 = F32_MANTISSA_LIMIT - 1;
#[cfg(feature = "checked-cast")]
pub(crate) const F32_MANTISSA_LIMIT_PLUS_1: u32 = F32_MANTISSA_LIMIT + 1;

#[cfg(feature = "checked-cast")]
pub(crate) const F64_MANTISSA_LIMIT: u64 = 1 << f64::MANTISSA_DIGITS;
#[cfg(feature = "checked-cast")]
pub(crate) const F64_MANTISSA_LIMIT_MINUS_1: u64 = F64_MANTISSA_LIMIT - 1;
#[cfg(feature = "checked-cast")]
pub(crate) const F64_MANTISSA_LIMIT_PLUS_1: u64 = F64_MANTISSA_LIMIT + 1;

// ========== saturating-cast 用の定数・マクロ ==========

// `u128 → f32` で `as` が ∞ に丸める最小の値（2^128 − 2^103）。
// f32::MAX（2^128 − 2^104）と 2^128 のちょうど中点で、最近接偶数丸めにより
// これ未満は f32::MAX へ丸められ、これ以上は ∞ に丸まる。
// 有効ビット幅が 25 なので f64 では正確に表現でき、f64 → f32 側の飽和境界テスト
// にも使い回せる。
#[cfg(feature = "saturating-cast")]
pub(crate) const U128_TO_F32_OVERFLOW_MIDPOINT: u128 = u128::MAX - (1 << 103) + 1;

// F32S / F64S から NaN を除き昇順に並べ替えた配列。
// 単調性（順序保存）プロパティの隣接ペア検証に使う。
#[cfg(feature = "saturating-cast")]
pub(crate) const F32S_ASCENDING: [f32; 14] = [
    f32::NEG_INFINITY,
    f32::MIN,
    -1e30,
    -42.5,
    -5.0,
    -3.9,
    -0.0,
    0.0,
    3.9,
    42.5,
    300.0,
    1e30,
    f32::MAX,
    f32::INFINITY,
];
#[cfg(feature = "saturating-cast")]
pub(crate) const F64S_ASCENDING: [f64; 16] = [
    f64::NEG_INFINITY,
    f64::MIN,
    -1e300,
    -1e30,
    -42.5,
    -5.0,
    -3.9,
    -0.0,
    0.0,
    3.9,
    42.5,
    300.0,
    1e30,
    1e300,
    f64::MAX,
    f64::INFINITY,
];

// 昇順のソース配列を隣接ペアで走査し、キャスト結果の順序保存
// （`v1 <= v2` なら `cast(v1) <= cast(v2)`）を確認する。
// `as` の回り込みでは成立しない、飽和キャスト固有の性質。
#[cfg(feature = "saturating-cast")]
macro_rules! check_monotonic {
    ($m:ident; $($arr:expr),+ $(,)?) => {
        $(
            for pair in $arr.windows(2) {
                let (v1, v2) = (pair[0], pair[1]);
                assert!(v1 <= v2, "test data must be ascending: {:?}, {:?}", v1, v2);
                assert!(
                    v1.$m() <= v2.$m(),
                    "order not preserved: {:?} -> {:?}, {:?} -> {:?}",
                    v1,
                    v1.$m(),
                    v2,
                    v2.$m(),
                );
            }
        )+
    };
}

#[cfg(feature = "saturating-cast")]
pub(crate) use check_monotonic;
