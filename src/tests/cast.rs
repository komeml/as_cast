//! `CastF32` / `CastF64` のテスト。
//!
//! 実装は `self as f32` / `self as f64` の薄いラッパなので、各テストは
//! 「トレイト経由の結果」を「`as` の結果」または既知の期待値とビット単位で比較する。
//! NaN は `is_nan()`、符号付きゼロは符号ビット（`to_bits` / `is_sign_*`）で扱う。

use crate::cast::{CastF32, CastF64};

// ========== 比較ヘルパ ==========
// 浮動小数点を to_bits() で厳密比較する（±0.0 を区別し、clippy::float_cmp を回避）。
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

// 各整数型の MIN / MAX をまとめて検証（符号なしの MIN は 0）。
macro_rules! check_bounds {
    ($($t:ty),* $(,)?) => {$(
        check_int!(<$t>::MIN);
        check_int!(<$t>::MAX);
    )*};
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

/// 符号なし整数
#[test]
fn unsigned_integers() {
    // 0（= 各型の MIN）と 各型の MAX
    check_bounds!(u8, u16, u32, u64, u128, usize);

    // 中間の半端な値（u8/u16 には入らないので u32 以上）
    check_int!(123_456_789u32);
    check_int!(123_456_789u64);
    check_int!(123_456_789u128);
    check_int!(123_456_789usize);

    // 2^24 近辺（f32 の丸め境界）— 保持できる型のみ
    check_int!(16_777_216u32);
    check_int!(16_777_217u32);
    check_int!(16_777_216u64);
    check_int!(16_777_217u64);
    check_int!(16_777_216u128);
    check_int!(16_777_217u128);
    check_int!(16_777_216usize);
    check_int!(16_777_217usize);
}

/// 符号あり整数
#[test]
fn signed_integers() {
    // 各型の MIN / MAX
    check_bounds!(i8, i16, i32, i64, i128, isize);

    // 0 と -1（全型）
    check_int!(0i8);
    check_int!(0i16);
    check_int!(0i32);
    check_int!(0i64);
    check_int!(0i128);
    check_int!(0isize);
    check_int!(-1i8);
    check_int!(-1i16);
    check_int!(-1i32);
    check_int!(-1i64);
    check_int!(-1i128);
    check_int!(-1isize);

    // 負の半端な値（i8/i16 には入らないので i32 以上）
    check_int!(-123_456_789i32);
    check_int!(-123_456_789i64);
    check_int!(-123_456_789i128);
    check_int!(-123_456_789isize);
}

/// 浮動小数点（通常値）
#[test]
#[allow(clippy::approx_constant)]
fn floats() {
    // 0.0 / -0.0（符号ビットも to_bits で区別される）
    check_from_f32!(0.0);
    check_from_f32!(-0.0);
    check_from_f64!(0.0);
    check_from_f64!(-0.0);

    // 小数を持つ値
    check_from_f32!(1.5);
    check_from_f32!(3.14);
    check_from_f64!(1.5);
    check_from_f64!(3.14);

    // 各型の MIN / MAX（有限）
    check_from_f32!(f32::MIN);
    check_from_f32!(f32::MAX);
    check_from_f64!(f64::MIN);
    check_from_f64!(f64::MAX);

    // 無限大
    check_from_f32!(f32::INFINITY);
    check_from_f32!(f32::NEG_INFINITY);
    check_from_f64!(f64::INFINITY);
    check_from_f64!(f64::NEG_INFINITY);
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
