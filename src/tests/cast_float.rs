//! `CastF32` / `CastF64` のテスト。
//!
//! 実装は `self as f32` / `self as f64` の薄いラッパなので、各テストは
//! 「トレイト経由の結果」を「`as` の結果」または既知の期待値とビット単位で比較する。
//! NaN は `is_nan()`、符号付きゼロは符号ビット（`to_bits` / `is_sign_*`）で扱う。

use super::cast_utility::{
    F32S, F64S, I8S, I16S, I32S, I64S, I128S, ISIZES, U8S, U16S, U32S, U64S, U128S, USIZES,
};
use crate::cast::{CastF32, CastF64};

/// 浮動小数点を `to_bits()` で厳密比較する（±0.0 を区別し、`clippy::float_cmp` を回避）。
///
/// `cast_f32` / `cast_f64` の結果を `as` の結果や既知の期待値とビット単位で突き合わせる際に使う。
macro_rules! assert_bits_eq {
    ($actual:expr, $expected:expr $(,)?) => {{
        let (a, e) = ($actual, $expected);
        assert_eq!(
            a.to_bits(),
            e.to_bits(),
            "got {:?} (0x{:x}), expected {:?} (0x{:x})",
            a,
            a.to_bits(),
            e,
            e.to_bits(),
        );
    }};
}

// 整数値 1 つについて cast_f32 / cast_f64 が `as` と一致するか検証（オラクル）。
macro_rules! check_int {
    ($val:expr) => {{
        let v = $val;
        assert_bits_eq!(v.cast_f32(), v as f32);
        assert_bits_eq!(v.cast_f64(), v as f64);
    }};
}

// 共有のソース値配列を走査し、全要素で `check_int!` を回す。
macro_rules! check_ints {
    ($($arr:expr),+ $(,)?) => {$(
        for &v in $arr.iter() {
            check_int!(v);
        }
    )+};
}

// f32 入力: cast_f64 は拡大。
macro_rules! check_from_f32 {
    ($val:expr) => {{
        let v: f32 = $val;
        assert_bits_eq!(v.cast_f64(), v as f64);
    }};
}

// f64 入力: cast_f32 は縮小。
macro_rules! check_from_f64 {
    ($val:expr) => {{
        let v: f64 = $val;
        assert_bits_eq!(v.cast_f32(), v as f32);
    }};
}

/// 符号なし整数（各型の MIN / 中間 / MAX を共有配列から網羅）
#[test]
fn unsigned_integers() {
    check_ints!(U8S, U16S, U32S, U64S, U128S, USIZES);
}

/// 符号あり整数（各型の MIN / -1 / 0 / 中間 / MAX を共有配列から網羅）
#[test]
fn signed_integers() {
    check_ints!(I8S, I16S, I32S, I64S, I128S, ISIZES);
}

/// 浮動小数点（通常値・境界・∞・NaN を共有配列から網羅）
#[test]
fn floats() {
    // f32 入力（0.0 / -0.0 / 小数 / MIN / MAX / ∞ / NaN）→ cast_f64 は拡大。
    for &v in F32S.iter() {
        check_from_f32!(v);
    }
    // f64 入力 → cast_f32 は縮小。
    for &v in F64S.iter() {
        check_from_f64!(v);
    }
}

/// f32 の丸め境界（自己文書化）
#[test]
fn f32_rounding_boundary_at_2pow24() {
    // 2^24 は f32 が連続して表せる最大の整数 → そのまま
    assert_bits_eq!(16_777_216u32.cast_f32(), 16_777_216.0_f32);
    // 2^24 + 1 は表現できず、最近接偶数丸めで 2^24 に落ちる
    assert_bits_eq!(16_777_217u32.cast_f32(), 16_777_216.0_f32);
    // f64 は 53bit 仮数なのでどちらも正確
    assert_bits_eq!(16_777_216u32.cast_f64(), 16_777_216.0_f64);
    assert_bits_eq!(16_777_217u32.cast_f64(), 16_777_217.0_f64);
}

/// f64 → f32 のオーバーフロー（自己文書化）
#[test]
fn f64_to_f32_overflow_is_infinity() {
    // f64 の有限 MAX/MIN は f32 の範囲を超える → ±∞
    assert_bits_eq!(f64::MAX.cast_f32(), f32::INFINITY);
    assert_bits_eq!(f64::MIN.cast_f32(), f32::NEG_INFINITY);
    // ∞ はそのまま ∞
    assert_bits_eq!(f64::INFINITY.cast_f32(), f32::INFINITY);
    assert_bits_eq!(f32::INFINITY.cast_f64(), f64::INFINITY);
}

/// 符号付きゼロの保存
#[test]
fn signed_zero_is_preserved() {
    assert!((-0.0f32).cast_f64().is_sign_negative());
    assert!((-0.0f64).cast_f32().is_sign_negative());

    assert!((0.0f32).cast_f64().is_sign_positive());
    assert!((0.0f64).cast_f32().is_sign_positive());
}

/// NaN は NaN のまま
#[test]
fn nan_stays_nan() {
    assert!(f32::NAN.cast_f64().is_nan());
    assert!(f64::NAN.cast_f32().is_nan());
}
