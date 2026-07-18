//! `StrictCastU8` … `StrictCastIsize`（整数型への変換）のテスト。
//!
//! `checked_cast_int.rs` のミラー。StrictCast* は CheckedCast* と同じ判定
//! （`convert_int_to_int!` / `convert_float_to_int!`）を共有し、損失時に
//! `None` を返す代わりに panic するため、テスト内容は「`None` の検証」を
//! `#[should_panic(expected = "strict cast failed")]` に置き換えた構成になっている。
//! 境界値の選定とオラクルの使い分け原則は `checked_cast_int.rs` の
//! モジュールコメントを参照。
//!
//! テストは 2 層構成:
//!
//! 1. 網羅層（全 impl の生成保証）:
//!    - 整数 → 整数: `try_from` が `Ok` を返す値についてのみ
//!      `v.strict_cast_xx()` が同じ値になることを確認する。`Err` になる値は
//!      panic するため走査から除外する（no_std クレートの単体テストでは
//!      `catch_unwind` が使えない）。panic 側はゴールデン層で固定する。
//!    - 浮動小数点 → 整数: `F32S` / `F64S` は損失値を含み走査できないため、
//!      整数値ぴったりの値を全ターゲットへ変換して impl の生成を保証する。
//! 2. ゴールデン層: 成功 / panic の境界を固定値で押さえる。
//!    1 テストで検証できる panic は 1 箇所だけなので、panic 側は
//!    `panic_tests!` マクロで 1 ケース = 1 テスト関数に分割する。
//!
//! なお恒等変換（例: `u8` の `strict_cast_u8`）は実装されていないため、網羅層では
//! 変換先と同じ型の入力は除外する。

use super::cast_utility::{
    I8S, I16S, I32S, I64S, I128S, ISIZES, U8S, U16S, U32S, U64S, U128S, USIZES,
};
use crate::strict_cast::{
    StrictCastI8, StrictCastI16, StrictCastI32, StrictCastI64, StrictCastI128, StrictCastIsize,
    StrictCastU8, StrictCastU16, StrictCastU32, StrictCastU64, StrictCastU128, StrictCastUsize,
};

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

// 変換先 `$t`（メソッド `$m`）について、渡された各整数ソース配列のうち
// `<$t>::try_from(v)` が `Ok` になる要素で `v.$m() == 変換値` を確認する。
// `Err` になる要素は panic するため走査から除外する（panic 側はゴールデン層で固定）。
macro_rules! check_try_from_agreement_to {
    ($m:ident, $t:ty; $($arr:expr),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                // 拡大変換では `try_from` が infallible になり `if let Ok` が
                // irrefutable 警告になるため、いったん Option に変換する
                let expected = <$t>::try_from(v).ok();
                if let Some(expected) = expected {
                    assert_eq!(
                        v.$m(),
                        expected,
                        "source {:?} -> {}",
                        v,
                        stringify!($t),
                    );
                }
            }
        )+
    };
}

/// 整数 → 符号なし整数が `try_from` の成功側と一致する（全ソース整数型を網羅）。
#[test]
fn try_from_agreement_to_unsigned() {
    check_try_from_agreement_to!(strict_cast_u8, u8;
        U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_u16, u16;
        U8S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_u32, u32;
        U8S, U16S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_u64, u64;
        U8S, U16S, U32S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_u128, u128;
        U8S, U16S, U32S, U64S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_usize, usize;
        U8S, U16S, U32S, U64S, U128S,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
}

/// 整数 → 符号あり整数が `try_from` の成功側と一致する（全ソース整数型を網羅）。
#[test]
fn try_from_agreement_to_signed() {
    check_try_from_agreement_to!(strict_cast_i8, i8;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_i16, i16;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I32S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_i32, i32;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I64S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_i64, i64;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I128S, ISIZES);
    check_try_from_agreement_to!(strict_cast_i128, i128;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, ISIZES);
    check_try_from_agreement_to!(strict_cast_isize, isize;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S);
}

// `$v`（整数値 42 ぴったりの浮動小数点）を全整数ターゲットへ変換し、
// すべて 42 になることを確認する（float ソースの全 impl 生成保証）。
macro_rules! check_42_for_all_int_targets {
    ($v:expr) => {{
        let v = $v;
        assert_eq!(v.strict_cast_u8(), 42);
        assert_eq!(v.strict_cast_u16(), 42);
        assert_eq!(v.strict_cast_u32(), 42);
        assert_eq!(v.strict_cast_u64(), 42);
        assert_eq!(v.strict_cast_u128(), 42);
        assert_eq!(v.strict_cast_usize(), 42);
        assert_eq!(v.strict_cast_i8(), 42);
        assert_eq!(v.strict_cast_i16(), 42);
        assert_eq!(v.strict_cast_i32(), 42);
        assert_eq!(v.strict_cast_i64(), 42);
        assert_eq!(v.strict_cast_i128(), 42);
        assert_eq!(v.strict_cast_isize(), 42);
    }};
}

/// 浮動小数点 → 全整数ターゲットの成功側網羅（全 impl の生成保証）。
#[test]
fn float_exact_to_all_int_targets() {
    check_42_for_all_int_targets!(42.0f32);
    check_42_for_all_int_targets!(42.0f64);
}

// ========== 2. 整数 → 整数のゴールデン ==========

/// 縮小: 境界ちょうどは成功（+1 超えは panic 側ゴールデンで固定）。
#[test]
fn int_narrowing_boundary_ok() {
    assert_eq!(255u16.strict_cast_u8(), 255);
    assert_eq!((-128i16).strict_cast_i8(), -128);
}

/// 符号なし → 符号あり同幅: ターゲットの MAX ちょうどは成功。
#[test]
fn int_unsigned_to_signed_same_width_ok() {
    assert_eq!(127u8.strict_cast_i8(), 127);
}

/// 拡大変換は常に成功（値保存）。
#[test]
fn int_widening_ok() {
    assert_eq!(u8::MAX.strict_cast_u128(), 255);
    assert_eq!(i8::MIN.strict_cast_i128(), -128);
    assert_eq!(i8::MAX.strict_cast_i128(), 127);
}

/// usize: ポインタ幅に依存しない成功ケース。
#[test]
fn int_usize_ok() {
    assert_eq!(0usize.strict_cast_u8(), 0);
}

panic_tests! {
    // 縮小: 境界 +1 超え
    int_narrowing_over_max_panics: 256u16 => strict_cast_u8;
    int_narrowing_under_min_panics: (-129i16) => strict_cast_i8;
    // 符号なし → 符号あり同幅: ターゲットの MAX 超え
    int_unsigned_to_signed_same_width_panics: 128u8 => strict_cast_i8;
    // 負値 → 符号なしはすべて panic（`as` なら回り込む値なので、回り込み検出の証明になる）
    int_negative_to_u8_panics: (-1i8) => strict_cast_u8;
    int_negative_to_u16_panics: (-1i8) => strict_cast_u16;
    int_negative_to_u32_panics: (-1i8) => strict_cast_u32;
    int_negative_to_u64_panics: (-1i8) => strict_cast_u64;
    int_negative_to_u128_panics: (-1i8) => strict_cast_u128;
    int_negative_to_usize_panics: (-1i8) => strict_cast_usize;
    // `u128::MAX` はどのポインタ幅でも usize に収まらない
    int_u128_max_to_usize_panics: u128::MAX => strict_cast_usize;
}

// ========== 3. 浮動小数点 → 整数のゴールデン ==========

/// 整数値ぴったりの浮動小数点は成功。
#[test]
fn float_exact_integer_ok() {
    assert_eq!(42.0f32.strict_cast_i32(), 42);
    assert_eq!((-5.0f64).strict_cast_i64(), -5);
}

/// -0.0 は `0.0 == -0.0` により 0 になる（仕様の文書化）。
#[test]
fn float_negative_zero_is_zero() {
    assert_eq!((-0.0f32).strict_cast_u8(), 0);
    assert_eq!((-0.0f64).strict_cast_i64(), 0);
}

/// MAX ガードの不発: ターゲット MAX ちょうどへの正当な変換が誤って panic しないこと。
#[test]
fn float_max_guard_does_not_misfire() {
    // u16::BITS(16) ≤ f32::MANTISSA_DIGITS(24) なのでガード対象外
    assert_eq!(65_535.0f32.strict_cast_u16(), u16::MAX);
    // u32::BITS(32) ≤ f64::MANTISSA_DIGITS(53)
    assert_eq!(4_294_967_295.0f64.strict_cast_u32(), u32::MAX);
    // f32::MAX = (2^24 − 1) × 2^104。cast 結果が u128::MAX に一致しないためガード不発
    assert_eq!(
        f32::MAX.strict_cast_u128(),
        340_282_346_638_528_859_811_704_183_484_516_925_440,
    );
    // ガードは MAX のみ見る。MIN（−2^31 / −2^63）は 2 の冪で正確に表現できるので成功
    assert_eq!((-2_147_483_648.0f32).strict_cast_i32(), i32::MIN);
    assert_eq!(
        (-9_223_372_036_854_775_808.0f64).strict_cast_i64(),
        i64::MIN
    );
}

/// 2^53 を超えても f64 が実際に保持している整数値はそのまま無損失で変換できる。
#[test]
fn float_beyond_mantissa_but_exact_ok() {
    // 1e30 の f64 近似値は整数（53bit 仮数 × 2^47）なので u128 に無損失で収まる
    assert_eq!(
        1e30f64.strict_cast_u128(),
        1_000_000_000_000_000_019_884_624_838_656,
    );
}

/// usize / isize ターゲットの代表ケース（成功側）。
#[test]
fn float_to_usize_isize_ok() {
    assert_eq!(42.0f32.strict_cast_usize(), 42);
    assert_eq!((-5.0f64).strict_cast_isize(), -5);
}

panic_tests! {
    // 小数部があれば panic（`as` は切り捨てるが strict は弾く、という差分の固定）
    float_fractional_positive_f32_panics: 3.9f32 => strict_cast_i32;
    float_fractional_negative_f32_panics: (-3.9f32) => strict_cast_i32;
    float_fractional_half_f32_panics: 42.5f32 => strict_cast_i32;
    float_fractional_positive_f64_panics: 3.9f64 => strict_cast_i64;
    float_fractional_negative_f64_panics: (-3.9f64) => strict_cast_i64;
    float_fractional_half_f64_panics: 42.5f64 => strict_cast_i64;
    // 負値 → 符号なし（`as` なら 0 に飽和する値）
    float_negative_to_unsigned_panics: (-1.0f32) => strict_cast_u8;
    float_negative_to_usize_panics: (-1.0f32) => strict_cast_usize;
    // 範囲外（`as` なら MAX に飽和する値）
    float_out_of_range_panics: 300.0f32 => strict_cast_u8;
    float_large_out_of_range_panics: 1e30f32 => strict_cast_u32;
}

panic_tests! {
    // NaN は全整数ターゲットで panic（早期 return の検証）: f32 ソース
    float_nan_f32_to_u8_panics: f32::NAN => strict_cast_u8;
    float_nan_f32_to_u16_panics: f32::NAN => strict_cast_u16;
    float_nan_f32_to_u32_panics: f32::NAN => strict_cast_u32;
    float_nan_f32_to_u64_panics: f32::NAN => strict_cast_u64;
    float_nan_f32_to_u128_panics: f32::NAN => strict_cast_u128;
    float_nan_f32_to_usize_panics: f32::NAN => strict_cast_usize;
    float_nan_f32_to_i8_panics: f32::NAN => strict_cast_i8;
    float_nan_f32_to_i16_panics: f32::NAN => strict_cast_i16;
    float_nan_f32_to_i32_panics: f32::NAN => strict_cast_i32;
    float_nan_f32_to_i64_panics: f32::NAN => strict_cast_i64;
    float_nan_f32_to_i128_panics: f32::NAN => strict_cast_i128;
    float_nan_f32_to_isize_panics: f32::NAN => strict_cast_isize;
    // NaN: f64 ソース
    float_nan_f64_to_u8_panics: f64::NAN => strict_cast_u8;
    float_nan_f64_to_u16_panics: f64::NAN => strict_cast_u16;
    float_nan_f64_to_u32_panics: f64::NAN => strict_cast_u32;
    float_nan_f64_to_u64_panics: f64::NAN => strict_cast_u64;
    float_nan_f64_to_u128_panics: f64::NAN => strict_cast_u128;
    float_nan_f64_to_usize_panics: f64::NAN => strict_cast_usize;
    float_nan_f64_to_i8_panics: f64::NAN => strict_cast_i8;
    float_nan_f64_to_i16_panics: f64::NAN => strict_cast_i16;
    float_nan_f64_to_i32_panics: f64::NAN => strict_cast_i32;
    float_nan_f64_to_i64_panics: f64::NAN => strict_cast_i64;
    float_nan_f64_to_i128_panics: f64::NAN => strict_cast_i128;
    float_nan_f64_to_isize_panics: f64::NAN => strict_cast_isize;
}

panic_tests! {
    // ±∞ は panic（`as` なら MAX / MIN に飽和する値）
    float_infinity_f32_to_i32_panics: f32::INFINITY => strict_cast_i32;
    float_neg_infinity_f32_to_i32_panics: f32::NEG_INFINITY => strict_cast_i32;
    float_infinity_f32_to_u8_panics: f32::INFINITY => strict_cast_u8;
    float_neg_infinity_f32_to_u8_panics: f32::NEG_INFINITY => strict_cast_u8;
    float_infinity_f64_to_i64_panics: f64::INFINITY => strict_cast_i64;
    float_neg_infinity_f64_to_i64_panics: f64::NEG_INFINITY => strict_cast_i64;
}

panic_tests! {
    // MAX ガードの発火（このモジュールで最重要）: 飽和した cast の round-trip が
    // 偽一致するケースを `!(cast == MAX && BITS > MANTISSA_DIGITS)` ガードが正しく弾く。
    // 2^32: u32 へは飽和で u32::MAX になり、`u32::MAX as f32` も 2^32 に丸まって偽一致する
    float_max_guard_u32_panics: 4_294_967_296.0f32 => strict_cast_u32;
    // 2^64: 同上（u64 版）
    float_max_guard_u64_panics: 18_446_744_073_709_551_616.0f32 => strict_cast_u64;
    // ∞ は u128::MAX に飽和し、`u128::MAX as f32` も ∞ に丸まって偽一致する
    float_max_guard_u128_panics: f32::INFINITY => strict_cast_u128;
}
