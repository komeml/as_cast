//! `StrictCastF32` / `StrictCastF64`（浮動小数点型への変換）のテスト。
//!
//! `checked_cast_float.rs` のミラー。StrictCastF32 / StrictCastF64 は
//! CheckedCastF32 / CheckedCastF64 と同じ判定（`can_convert_int_to_float!` /
//! `convert_float_to_float!`）を共有し、損失時に `None` を返す代わりに
//! panic するため、テスト内容は「`None` の検証」を
//! `#[should_panic(expected = "strict cast failed")]` に置き換えた構成になっている。
//! 境界値の選定は `checked_cast_float.rs` と同一。
//!
//! テストは 2 層構成:
//!
//! 1. 網羅層（全 impl の生成保証）: 常に無損失な値だけを走査し、結果が
//!    `v as xx` と一致することを確認する。損失し得る値は panic するため
//!    走査できない（no_std クレートの単体テストでは `catch_unwind` が
//!    使えない）。panic 側の境界はすべてゴールデン層で固定する。
//! 2. ゴールデン層: 仮数部境界・有効ビット幅・NaN / ∞ / subnormal といった
//!    成功 / panic の境界を固定値で押さえる。1 テストで検証できる panic は
//!    1 箇所だけなので、panic 側は `panic_tests!` マクロで
//!    1 ケース = 1 テスト関数に分割する。
//!
//! なお恒等変換（f32 の `strict_cast_f32`、f64 の `strict_cast_f64`）は実装されて
//! いないため、網羅層では変換先と同じ型の入力は除外する。

use super::cast_utility::{I8S, I16S, I32S, U8S, U16S, U32S};
use crate::strict_cast::{StrictCastF32, StrictCastF64};

// 浮動小数点が整数を連続で表せる上限（f32: 2^24、f64: 2^53）とその前後 ±1。
// `cast_utility` の同名定数は checked-cast フィーチャー限定のため、
// strict-cast 単独ビルドでも使えるようここに同値を定義する。
const F32_MANTISSA_LIMIT: u32 = 1 << f32::MANTISSA_DIGITS;
const F32_MANTISSA_LIMIT_MINUS_1: u32 = F32_MANTISSA_LIMIT - 1;
const F32_MANTISSA_LIMIT_PLUS_1: u32 = F32_MANTISSA_LIMIT + 1;

const F64_MANTISSA_LIMIT: u64 = 1 << f64::MANTISSA_DIGITS;
const F64_MANTISSA_LIMIT_MINUS_1: u64 = F64_MANTISSA_LIMIT - 1;
const F64_MANTISSA_LIMIT_PLUS_1: u64 = F64_MANTISSA_LIMIT + 1;

// 1 行につき `#[should_panic(expected = "strict cast failed")]` 付きテスト関数を
// 1 つ生成する。1 テストで検証できる panic は 1 箇所だけなので、panic 側の
// ゴールデンはすべて個別のテスト関数に分割する。
macro_rules! panic_tests {
    ($($name:ident: $v:expr => $m:ident;)+) => {
        $(
            #[test]
            #[should_panic(expected = "strict cast failed")]
            fn $name() {
                let _ = ($v).$m();
            }
        )+
    };
}

// ========== 1. 網羅層 ==========

// 変換先 `$t`（メソッド `$m`）について、渡された各配列の全要素で
// `v.$m() == v as $t` を確認する。無損失が保証される（仮数部に必ず収まる）
// ソースにのみ使うこと。
macro_rules! check_eq_as {
    ($m:ident, $t:ty; $($arr:expr),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                assert_eq!(v.$m(), v as $t, "source {:?} -> {}", v, stringify!($t));
            }
        )+
    };
}

/// 仮数部に常に収まる整数型は全値成功（値セット全走査）。
#[test]
#[allow(clippy::float_cmp)]
fn int_small_types_never_panic() {
    // u8 / i8 / u16 / i16 → f32: 有効ビット幅は最大でも 16 ≤ 24
    check_eq_as!(strict_cast_f32, f32; U8S, I8S, U16S, I16S);
    // u32 / i32 → f64: 最大でも 32 ≤ 53
    check_eq_as!(strict_cast_f64, f64; U32S, I32S);
}

// 全整数ソース型で 0（ゼロの早期分岐）と 100（通常経路）を変換し、
// 全 impl の生成を保証する。どちらも常に無損失。
macro_rules! check_all_int_sources {
    ($m:ident; $($src:ty),+ $(,)?) => {
        $(
            assert_eq!((0 as $src).$m(), 0.0, "source type {}", stringify!($src));
            assert_eq!((100 as $src).$m(), 100.0, "source type {}", stringify!($src));
        )+
    };
}

/// 全ソース整数型の impl 生成保証（0 は早期分岐、100 は通常経路）。
#[test]
#[allow(clippy::float_cmp)]
fn int_all_sources_ok() {
    check_all_int_sources!(strict_cast_f32;
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    check_all_int_sources!(strict_cast_f64;
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}

// ========== 2. 整数 → 浮動小数点のゴールデン ==========

/// f32 の仮数部境界（2^24 ± 1）: 境界ちょうどまでは成功（+1 は panic 側で固定）。
#[test]
fn int_f32_mantissa_boundary_ok() {
    assert_eq!(F32_MANTISSA_LIMIT_MINUS_1.strict_cast_f32(), 16_777_215.0);
    assert_eq!(F32_MANTISSA_LIMIT.strict_cast_f32(), 16_777_216.0);
}

/// f64 の仮数部境界（2^53 ± 1）: 境界ちょうどまでは成功（+1 は panic 側で固定）。
#[test]
fn int_f64_mantissa_boundary_ok() {
    assert_eq!(
        F64_MANTISSA_LIMIT_MINUS_1.strict_cast_f64(),
        9_007_199_254_740_991.0,
    );
    assert_eq!(
        F64_MANTISSA_LIMIT.strict_cast_f64(),
        9_007_199_254_740_992.0,
    );
}

/// 有効ビット幅（最上位〜最下位の set bit 間の距離）による判定（成功側）。
/// 値の大きさではなく「仮数部に収まるビット幅か」で決まることを固定する。
#[test]
#[allow(clippy::float_cmp)]
fn int_effective_bit_width_ok() {
    // 2^100: 有効ビット幅 1 → 成功
    let v = 1u128 << 100;
    assert_eq!(v.strict_cast_f32(), v as f32);
    // 2^100 + 2^80: 幅 21 ≤ 24 → 成功
    let v = (1u128 << 100) + (1u128 << 80);
    assert_eq!(v.strict_cast_f32(), v as f32);
}

/// 符号あり整数は `unsigned_abs` 経由で判定される（成功側）。
#[test]
#[allow(clippy::float_cmp)]
fn int_signed_path_ok() {
    // abs = 2^24 − 1: 幅 24 ≤ 24 → 成功
    assert_eq!((-16_777_215i32).strict_cast_f32(), -16_777_215.0);
    // i64::MIN: abs = 2^63 で幅 1 → 成功
    assert_eq!(i64::MIN.strict_cast_f64(), i64::MIN as f64);
}

/// `unsigned_abs` のオーバーフロー安全性: abs（2^127）が i128 に収まらない
/// i128::MIN でも（strict cast の損失検知とは別の意味で）パニックせず変換できる。
#[test]
#[allow(clippy::float_cmp)]
fn int_i128_min_ok() {
    assert_eq!(i128::MIN.strict_cast_f32(), i128::MIN as f32);
}

panic_tests! {
    // f32 の仮数部境界 +1（2^24 + 1）
    int_f32_mantissa_limit_plus_1_panics: F32_MANTISSA_LIMIT_PLUS_1 => strict_cast_f32;
    // f64 の仮数部境界 +1（2^53 + 1）
    int_f64_mantissa_limit_plus_1_panics: F64_MANTISSA_LIMIT_PLUS_1 => strict_cast_f64;
    // 2^100 + 1: 有効ビット幅 101 > 24
    int_effective_bit_width_panics: ((1u128 << 100) + 1) => strict_cast_f32;
    // i64::MIN + 1: abs = 2^63 − 1 で幅 63 > 53
    int_signed_path_panics: (i64::MIN + 1) => strict_cast_f64;
    // round-trip オラクルでは検出できない偽陽性: u64::MAX as f32 は 2^64 に丸まり、
    // as u64 で飽和して u64::MAX に戻ってしまうが、有効ビット幅判定で正しく panic する
    int_round_trip_false_positive_u64_panics: u64::MAX => strict_cast_f32;
    // 同上: u128::MAX as f32 は ∞ に丸まり、as u128 で飽和して u128::MAX に戻ってしまう
    int_round_trip_false_positive_u128_panics: u128::MAX => strict_cast_f32;
}

// ========== 3. 浮動小数点間のゴールデン ==========

/// f32 → f64（拡大）: 有限値は常に成功。
#[test]
#[allow(clippy::float_cmp)]
fn f32_to_f64_finite_ok() {
    assert_eq!(0.1f32.strict_cast_f64(), 0.1f32 as f64);
    assert_eq!(1.5f32.strict_cast_f64(), 1.5);
    assert_eq!(f32::MAX.strict_cast_f64(), f32::MAX as f64);
}

/// f32 → f64: ±∞ は保存される。
#[test]
#[allow(clippy::float_cmp)]
fn f32_to_f64_infinity_ok() {
    assert_eq!(f32::INFINITY.strict_cast_f64(), f64::INFINITY);
    assert_eq!(f32::NEG_INFINITY.strict_cast_f64(), f64::NEG_INFINITY);
}

/// f64 → f32（縮小）: 正確に表現できる値は成功。
#[test]
#[allow(clippy::float_cmp)]
fn f64_to_f32_exact_ok() {
    assert_eq!(1.5f64.strict_cast_f32(), 1.5);
    assert_eq!((f32::MAX as f64).strict_cast_f32(), f32::MAX);
    // 2 の冪は指数部だけで表現できるので正確
    assert_eq!(1024.0f64.strict_cast_f32(), 1024.0);
}

/// f64 → f32: ±∞ は損失なし扱いで成功。
#[test]
#[allow(clippy::float_cmp)]
fn f64_to_f32_infinity_ok() {
    assert_eq!(f64::INFINITY.strict_cast_f32(), f32::INFINITY);
    assert_eq!(f64::NEG_INFINITY.strict_cast_f32(), f32::NEG_INFINITY);
}

/// f64 → f32: f32 の最小 subnormal に相当する値は正確に戻せるので成功。
#[test]
#[allow(clippy::float_cmp)]
fn f64_to_f32_subnormal_ok() {
    let min_subnormal = f32::from_bits(1);
    assert_eq!((min_subnormal as f64).strict_cast_f32(), min_subnormal);
}

/// f64 → f32: -0.0 は符号ビットを保って -0.0 になる。
/// `==` では +0.0 と区別できないため `to_bits` で確認する。
#[test]
fn f64_to_f32_negative_zero_keeps_sign_bit() {
    assert_eq!((-0.0f64).strict_cast_f32().to_bits(), (-0.0f32).to_bits());
}

panic_tests! {
    // 拡大でも NaN は panic（「widening なら安全」と思い込んだ改変への防波堤）
    f32_to_f64_nan_panics: f32::NAN => strict_cast_f64;
    // f32 に丸めると値が変わる
    f64_to_f32_rounding_panics: 0.1f64 => strict_cast_f32;
    // ∞ に丸まる
    f64_to_f32_overflow_max_panics: f64::MAX => strict_cast_f32;
    f64_to_f32_overflow_1e300_panics: 1e300f64 => strict_cast_f32;
    // 0 に丸まる（アンダーフロー）
    f64_to_f32_underflow_panics: 1e-300f64 => strict_cast_f32;
    // 縮小でも NaN は panic
    f64_to_f32_nan_panics: f64::NAN => strict_cast_f32;
}
