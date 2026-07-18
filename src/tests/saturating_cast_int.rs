//! `SaturatingCastU8` … `SaturatingCastIsize`（整数型への変換）のテスト。
//!
//! テストは 3 層構成:
//!
//! 1. 網羅層（全 impl の生成保証）:
//!    - 整数 → 整数: `try_from` が Ok ならその値、Err なら負は `MIN`・正は `MAX` を
//!      期待値とする。判定基準としては実装（`saturating_cast_clamp_both!`）と
//!      ほぼ同語反復だが、既存テストと同様「マクロが全型に impl を生成したこと」の
//!      保証を目的とする（`simple_as` の拡大 impl は常に Ok 側を通る）。
//!    - 浮動小数点 → 整数: `v as xx` との完全一致を確認する。Rust 1.45+ の `as`
//!      自体が飽和キャスト（NaN は 0）なので、これは同語反復でない完全なオラクルになる。
//! 2. ゴールデン層: 飽和が発火する / しない境界を固定値で押さえる。
//! 3. プロパティ層（単調性）: `v1 <= v2` なら `cast(v1) <= cast(v2)`。`as` の回り込み
//!    （例: `255u16 as u8 == 255` だが `256u16 as u8 == 0`）では成立しない飽和キャスト
//!    固有の性質なので、実装が単純 `as` に退化した場合を非同語反復に検出できる。
//!
//! なお恒等変換（例: `u8` の `saturating_cast_u8`）は実装されていないため、
//! 各層とも変換先と同じ型の入力は除外する。また符号なしソースでは実装の
//! `$v < 0` 分岐がデッドコードのため、テストでも踏むことはできない
//! （実装に `#[allow(unused_comparisons)]` があるのはそのため）。
//! usize / isize のゴールデンはポインタ幅に依存しないケースのみ固定する
//! （網羅層の `try_from` オラクルはポインタ幅に依存せず動く）。

use super::cast_utility::{
    F32S, F32S_ASCENDING, F64S, F64S_ASCENDING, I8S, I16S, I32S, I64S, I128S, ISIZES, U8S, U16S,
    U32S, U64S, U128S, USIZES, check_monotonic,
};
use crate::saturating_cast::{
    SaturatingCastI8, SaturatingCastI16, SaturatingCastI32, SaturatingCastI64, SaturatingCastI128,
    SaturatingCastIsize, SaturatingCastU8, SaturatingCastU16, SaturatingCastU32, SaturatingCastU64,
    SaturatingCastU128, SaturatingCastUsize,
};

// 符号判定のヘルパ。ジェネリックな比較にすることで、符号なし型に対する
// `v < 0` が引き起こす unused_comparisons 警告を避ける。
fn is_negative<T: Default + PartialOrd>(v: T) -> bool {
    v < T::default()
}

// ========== 1. 網羅層 ==========

// 変換先 `$t`（メソッド `$m`）について、渡された各整数ソース配列の全要素で
// 「`try_from` が Ok ならその値、Err なら負は MIN・正は MAX」と一致することを確認する。
macro_rules! check_clamped_try_from_to {
    ($m:ident, $t:ty; $($arr:expr),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                let expected = match <$t>::try_from(v) {
                    Ok(x) => x,
                    Err(_) if is_negative(v) => <$t>::MIN,
                    Err(_) => <$t>::MAX,
                };
                assert_eq!(v.$m(), expected, "source {:?} -> {}", v, stringify!($t));
            }
        )+
    };
}

/// 整数 → 符号なし整数がクランプ付き `try_from` オラクルと一致する
/// （全ソース整数型を網羅）。
#[test]
fn clamped_try_from_agreement_to_unsigned() {
    check_clamped_try_from_to!(saturating_cast_u8, u8;
        U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_u16, u16;
        U8S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_u32, u32;
        U8S, U16S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_u64, u64;
        U8S, U16S, U32S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_u128, u128;
        U8S, U16S, U32S, U64S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_usize, usize;
        U8S, U16S, U32S, U64S, U128S,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
}

/// 整数 → 符号あり整数がクランプ付き `try_from` オラクルと一致する
/// （全ソース整数型を網羅）。
#[test]
fn clamped_try_from_agreement_to_signed() {
    check_clamped_try_from_to!(saturating_cast_i8, i8;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I16S, I32S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_i16, i16;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I32S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_i32, i32;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I64S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_i64, i64;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I128S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_i128, i128;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, ISIZES);
    check_clamped_try_from_to!(saturating_cast_isize, isize;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S);
}

// 変換先 `$t`（メソッド `$m`）について、F32S / F64S の全要素
// （NaN / ±∞ / 範囲外を含む）で `v.$m() == v as $t` を確認する。
macro_rules! check_float_as_agreement_to {
    ($($m:ident => $t:ty),+ $(,)?) => {
        $(
            for &v in F32S.iter() {
                assert_eq!(v.$m(), v as $t, "source {:?}f32 -> {}", v, stringify!($t));
            }
            for &v in F64S.iter() {
                assert_eq!(v.$m(), v as $t, "source {:?}f64 -> {}", v, stringify!($t));
            }
        )+
    };
}

/// 浮動小数点 → 整数が `as` と完全一致する（全 12 整数ターゲットを網羅）。
/// NaN / ±∞ / 範囲外を含む配列の全走査で、この経路の仕様をほぼ固定できる。
#[test]
fn float_as_agreement_to_int_targets() {
    check_float_as_agreement_to!(
        saturating_cast_u8 => u8,
        saturating_cast_u16 => u16,
        saturating_cast_u32 => u32,
        saturating_cast_u64 => u64,
        saturating_cast_u128 => u128,
        saturating_cast_usize => usize,
        saturating_cast_i8 => i8,
        saturating_cast_i16 => i16,
        saturating_cast_i32 => i32,
        saturating_cast_i64 => i64,
        saturating_cast_i128 => i128,
        saturating_cast_isize => isize,
    );
}

// ========== 2. 整数 → 整数のゴールデン ==========

/// 縮小・上方飽和: ターゲット範囲を超える正値は MAX に飽和する。
#[test]
fn int_narrowing_saturates_to_max() {
    assert_eq!(256u16.saturating_cast_u8(), 255);
    assert_eq!(u16::MAX.saturating_cast_u8(), 255);
}

/// 縮小・境界ちょうどは飽和せず値が保存される。
#[test]
fn int_narrowing_boundary_is_exact() {
    assert_eq!(255u16.saturating_cast_u8(), 255);
    assert_eq!((-128i16).saturating_cast_i8(), -128);
}

/// 縮小・下方飽和: ターゲット範囲を下回る負値は MIN に飽和する。
#[test]
fn int_narrowing_saturates_to_min() {
    assert_eq!((-129i16).saturating_cast_i8(), -128);
    assert_eq!(i16::MIN.saturating_cast_i8(), -128);
}

/// 負値 → 符号なしはすべて 0
/// （`as` なら回り込む値なので、回り込みが起きないことの証明になる）。
#[test]
fn int_negative_to_unsigned_is_zero() {
    assert_eq!((-1i8).saturating_cast_u8(), 0);
    assert_eq!((-1i8).saturating_cast_u16(), 0);
    assert_eq!((-1i8).saturating_cast_u32(), 0);
    assert_eq!((-1i8).saturating_cast_u64(), 0);
    assert_eq!((-1i8).saturating_cast_u128(), 0);
    assert_eq!((-1i8).saturating_cast_usize(), 0);
}

/// 符号なし → 符号あり同幅: ターゲットの MAX を超えると飽和する。
#[test]
fn int_unsigned_to_signed_same_width() {
    assert_eq!(127u8.saturating_cast_i8(), 127);
    assert_eq!(128u8.saturating_cast_i8(), 127);
}

/// 符号あり → 符号なし拡大: 上方飽和は発生し得ず、下方のみの経路。
#[test]
fn int_signed_to_unsigned_widening() {
    assert_eq!((-1i8).saturating_cast_u128(), 0);
    assert_eq!(127i8.saturating_cast_u128(), 127);
}

/// 極値の一括確認: `u128::MAX` / `i128::MIN` / `i128::MAX` が全ターゲットで
/// それぞれ MAX / MIN（符号なしは 0）/ MAX（縮小先）に飽和する。
#[test]
fn int_extreme_values_saturate() {
    // u128::MAX → 全ターゲットで各 MAX（u128 自身は恒等変換のため対象外）
    assert_eq!(u128::MAX.saturating_cast_u8(), u8::MAX);
    assert_eq!(u128::MAX.saturating_cast_u16(), u16::MAX);
    assert_eq!(u128::MAX.saturating_cast_u32(), u32::MAX);
    assert_eq!(u128::MAX.saturating_cast_u64(), u64::MAX);
    assert_eq!(u128::MAX.saturating_cast_usize(), usize::MAX);
    assert_eq!(u128::MAX.saturating_cast_i8(), i8::MAX);
    assert_eq!(u128::MAX.saturating_cast_i16(), i16::MAX);
    assert_eq!(u128::MAX.saturating_cast_i32(), i32::MAX);
    assert_eq!(u128::MAX.saturating_cast_i64(), i64::MAX);
    assert_eq!(u128::MAX.saturating_cast_i128(), i128::MAX);
    assert_eq!(u128::MAX.saturating_cast_isize(), isize::MAX);

    // i128::MIN → 符号ありは各 MIN、符号なしは 0
    assert_eq!(i128::MIN.saturating_cast_i8(), i8::MIN);
    assert_eq!(i128::MIN.saturating_cast_i16(), i16::MIN);
    assert_eq!(i128::MIN.saturating_cast_i32(), i32::MIN);
    assert_eq!(i128::MIN.saturating_cast_i64(), i64::MIN);
    assert_eq!(i128::MIN.saturating_cast_isize(), isize::MIN);
    assert_eq!(i128::MIN.saturating_cast_u8(), 0);
    assert_eq!(i128::MIN.saturating_cast_u16(), 0);
    assert_eq!(i128::MIN.saturating_cast_u32(), 0);
    assert_eq!(i128::MIN.saturating_cast_u64(), 0);
    assert_eq!(i128::MIN.saturating_cast_u128(), 0);
    assert_eq!(i128::MIN.saturating_cast_usize(), 0);

    // i128::MAX → 縮小先で各 MAX（u128 へは収まるため値保存）
    assert_eq!(i128::MAX.saturating_cast_i8(), i8::MAX);
    assert_eq!(i128::MAX.saturating_cast_i16(), i16::MAX);
    assert_eq!(i128::MAX.saturating_cast_i32(), i32::MAX);
    assert_eq!(i128::MAX.saturating_cast_i64(), i64::MAX);
    assert_eq!(i128::MAX.saturating_cast_isize(), isize::MAX);
    assert_eq!(i128::MAX.saturating_cast_u8(), u8::MAX);
    assert_eq!(i128::MAX.saturating_cast_u16(), u16::MAX);
    assert_eq!(i128::MAX.saturating_cast_u32(), u32::MAX);
    assert_eq!(i128::MAX.saturating_cast_u64(), u64::MAX);
    assert_eq!(i128::MAX.saturating_cast_usize(), usize::MAX);
    assert_eq!(i128::MAX.saturating_cast_u128(), i128::MAX as u128);
}

/// 拡大変換は値保存（`simple_as` 経路のゴールデン）。
#[test]
fn int_widening_preserves_value() {
    assert_eq!(u8::MAX.saturating_cast_u128(), 255);
    assert_eq!(i8::MIN.saturating_cast_i128(), -128);
}

/// usize / isize: ポインタ幅に依存しないケースのみ固定する。
#[test]
fn int_usize_isize_boundary() {
    assert_eq!(u128::MAX.saturating_cast_usize(), usize::MAX);
    assert_eq!(usize::MAX.saturating_cast_isize(), isize::MAX);
    assert_eq!((-1isize).saturating_cast_usize(), 0);
    assert_eq!(0usize.saturating_cast_u8(), 0);
}

// ========== 3. 浮動小数点 → 整数のゴールデン ==========
//
// 実装は `as` に委譲しているため、この層は「`as` が本当に飽和キャストである」
// ことを固定する回帰テスト（`cast_int` の自己文書化テストと同じ位置づけ）。

/// ゼロ方向への切り捨て（飽和なし）。
#[test]
fn float_truncates_toward_zero() {
    assert_eq!(3.9f32.saturating_cast_i32(), 3);
    assert_eq!((-3.9f32).saturating_cast_i32(), -3);
    assert_eq!(42.5f64.saturating_cast_i64(), 42);
}

/// 切り捨てと飽和の境目: 同じ結果（127）に 2 経路で到達する。
#[test]
fn float_truncation_vs_saturation_boundary() {
    // 切り捨て（127.99 の f32 近似値は 128 未満）
    assert_eq!(127.99f32.saturating_cast_i8(), 127);
    // 飽和
    assert_eq!(128.0f32.saturating_cast_i8(), 127);
}

/// 上方飽和。
#[test]
fn float_saturates_to_max() {
    assert_eq!(300.0f32.saturating_cast_u8(), 255);
    assert_eq!(1e30f32.saturating_cast_u32(), u32::MAX);
    assert_eq!(f64::MAX.saturating_cast_i64(), i64::MAX);
}

/// 下方飽和。
#[test]
fn float_saturates_to_min() {
    assert_eq!((-1.0f32).saturating_cast_u8(), 0);
    assert_eq!((-129.0f32).saturating_cast_i8(), -128);
    assert_eq!((-1e30f64).saturating_cast_i32(), i32::MIN);
}

/// 負の小数 → 符号なし: 切り捨てで 0 になる場合と飽和で 0 になる場合の対比。
#[test]
fn float_negative_fraction_to_unsigned() {
    // 切り捨てで 0（飽和ではない）
    assert_eq!((-0.5f32).saturating_cast_u8(), 0);
    // 飽和で 0
    assert_eq!((-1.5f32).saturating_cast_u8(), 0);
}

// `$v` を全 12 整数ターゲットへ変換し、すべて 0 になることを確認する
// （checked_cast の `check_none_for_all_int_targets!` の飽和版）。
macro_rules! check_zero_for_all_int_targets {
    ($v:expr) => {{
        let v = $v;
        assert_eq!(v.saturating_cast_u8(), 0);
        assert_eq!(v.saturating_cast_u16(), 0);
        assert_eq!(v.saturating_cast_u32(), 0);
        assert_eq!(v.saturating_cast_u64(), 0);
        assert_eq!(v.saturating_cast_u128(), 0);
        assert_eq!(v.saturating_cast_usize(), 0);
        assert_eq!(v.saturating_cast_i8(), 0);
        assert_eq!(v.saturating_cast_i16(), 0);
        assert_eq!(v.saturating_cast_i32(), 0);
        assert_eq!(v.saturating_cast_i64(), 0);
        assert_eq!(v.saturating_cast_i128(), 0);
        assert_eq!(v.saturating_cast_isize(), 0);
    }};
}

// `$v` を全 12 整数ターゲットへ変換し、すべて各ターゲットの MAX になることを確認する。
macro_rules! check_max_for_all_int_targets {
    ($v:expr) => {{
        let v = $v;
        assert_eq!(v.saturating_cast_u8(), u8::MAX);
        assert_eq!(v.saturating_cast_u16(), u16::MAX);
        assert_eq!(v.saturating_cast_u32(), u32::MAX);
        assert_eq!(v.saturating_cast_u64(), u64::MAX);
        assert_eq!(v.saturating_cast_u128(), u128::MAX);
        assert_eq!(v.saturating_cast_usize(), usize::MAX);
        assert_eq!(v.saturating_cast_i8(), i8::MAX);
        assert_eq!(v.saturating_cast_i16(), i16::MAX);
        assert_eq!(v.saturating_cast_i32(), i32::MAX);
        assert_eq!(v.saturating_cast_i64(), i64::MAX);
        assert_eq!(v.saturating_cast_i128(), i128::MAX);
        assert_eq!(v.saturating_cast_isize(), isize::MAX);
    }};
}

// `$v` を全 12 整数ターゲットへ変換し、符号ありは各 MIN・符号なしは 0 になることを
// 確認する。
macro_rules! check_min_or_zero_for_all_int_targets {
    ($v:expr) => {{
        let v = $v;
        assert_eq!(v.saturating_cast_u8(), 0);
        assert_eq!(v.saturating_cast_u16(), 0);
        assert_eq!(v.saturating_cast_u32(), 0);
        assert_eq!(v.saturating_cast_u64(), 0);
        assert_eq!(v.saturating_cast_u128(), 0);
        assert_eq!(v.saturating_cast_usize(), 0);
        assert_eq!(v.saturating_cast_i8(), i8::MIN);
        assert_eq!(v.saturating_cast_i16(), i16::MIN);
        assert_eq!(v.saturating_cast_i32(), i32::MIN);
        assert_eq!(v.saturating_cast_i64(), i64::MIN);
        assert_eq!(v.saturating_cast_i128(), i128::MIN);
        assert_eq!(v.saturating_cast_isize(), isize::MIN);
    }};
}

/// NaN は全整数ターゲットで 0。
#[test]
fn float_nan_is_zero() {
    check_zero_for_all_int_targets!(f32::NAN);
    check_zero_for_all_int_targets!(f64::NAN);
}

/// ±∞ は全整数ターゲットで MAX / MIN（符号なしは 0）に飽和する。
#[test]
fn float_infinity_saturates() {
    check_max_for_all_int_targets!(f32::INFINITY);
    check_max_for_all_int_targets!(f64::INFINITY);
    check_min_or_zero_for_all_int_targets!(f32::NEG_INFINITY);
    check_min_or_zero_for_all_int_targets!(f64::NEG_INFINITY);
}

/// -0.0 は 0。
#[test]
fn float_negative_zero_is_zero() {
    assert_eq!((-0.0f32).saturating_cast_u8(), 0);
    assert_eq!((-0.0f64).saturating_cast_i64(), 0);
}

/// 丸めと飽和の相互作用: `u32::MAX as f32` は 2^32 に丸まるが、u32 への飽和で
/// u32::MAX に戻る。checked_cast では「偽一致」として None に弾いた値が、
/// 飽和キャストでは正しく MAX になることの対比。
#[test]
fn float_rounding_saturation_interaction() {
    assert_eq!((u32::MAX as f32).saturating_cast_u32(), u32::MAX);
}

/// usize / isize ターゲットの代表ケース（ポインタ幅に依存しない値のみ）。
#[test]
fn float_to_usize_isize() {
    assert_eq!(42.0f32.saturating_cast_usize(), 42);
    assert_eq!((-1.0f32).saturating_cast_usize(), 0);
    assert_eq!((-5.0f64).saturating_cast_isize(), -5);
}

// ========== 4. プロパティ層（単調性） ==========

/// 単調性（順序保存）: 符号なし整数ターゲット（全ソース型を網羅）。
#[test]
fn monotonicity_to_unsigned() {
    check_monotonic!(saturating_cast_u8;
        U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_u16;
        U8S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_u32;
        U8S, U16S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_u64;
        U8S, U16S, U32S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_u128;
        U8S, U16S, U32S, U64S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_usize;
        U8S, U16S, U32S, U64S, U128S,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
}

/// 単調性（順序保存）: 符号あり整数ターゲット（全ソース型を網羅）。
#[test]
fn monotonicity_to_signed() {
    check_monotonic!(saturating_cast_i8;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_i16;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_i32;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I64S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_i64;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I128S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_i128;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, ISIZES,
        F32S_ASCENDING, F64S_ASCENDING);
    check_monotonic!(saturating_cast_isize;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S,
        F32S_ASCENDING, F64S_ASCENDING);
}
