//! `CastU8` … `CastIsize`（整数型への変換）のテスト。
//!
//! 実装はいずれも `self as <型>` の薄いラッパ。テストは 3 層構成:
//!
//! 1. `as` との一致（網羅）: 生成された各 impl について
//!    `v.cast_xx() == v as xx` を確認する。値の正しさより「マクロが全型に
//!    impl を生成したこと」の保証が目的。
//! 2. 整数 → 整数のゴールデン: `as` の非自明な挙動（下位ビット保持・
//!    ビット再解釈・境界での回り込み）を固定値で押さえる。
//! 3. 浮動小数点 → 整数のゴールデン: Rust 1.45+ の `as` 挙動
//!    （ゼロ方向への切り捨て + 飽和 + NaN → 0）を固定する。
//!
//! なお恒等変換（例: `u8` の `cast_u8`）は実装されていないため、網羅層では
//! 変換先と同じ型の入力は除外する。

use crate::cast::{
    CastI8, CastI16, CastI32, CastI64, CastI128, CastIsize, CastU8, CastU16, CastU32, CastU64,
    CastU128, CastUsize,
};

// ========== 1. `as` との一致（網羅） ==========

// 型ごとの入力値セット（符号なし・符号あり・float で分ける）。
// 各配列を「変換先と同じ型を除いた」全ソースとして網羅層に流し込む。
const U8S: [u8; 3] = [0, 200, u8::MAX];
const U16S: [u16; 3] = [0, 40_000, u16::MAX];
const U32S: [u32; 3] = [0, 3_000_000_000, u32::MAX];
const U64S: [u64; 3] = [0, 12_000_000_000_000, u64::MAX];
const U128S: [u128; 3] = [0, 340_000_000_000_000_000_000, u128::MAX];
const USIZES: [usize; 3] = [0, 100_000, usize::MAX];

const I8S: [i8; 6] = [i8::MIN, -100, -1, 0, 100, i8::MAX];
const I16S: [i16; 6] = [i16::MIN, -20_000, -1, 0, 20_000, i16::MAX];
const I32S: [i32; 6] = [i32::MIN, -2_000_000_000, -1, 0, 2_000_000_000, i32::MAX];
const I64S: [i64; 6] = [i64::MIN, -9_000_000_000, -1, 0, 9_000_000_000, i64::MAX];
const I128S: [i128; 6] = [
    i128::MIN,
    -170_000_000_000_000_000_000,
    -1,
    0,
    170_000_000_000_000_000_000,
    i128::MAX,
];
const ISIZES: [isize; 6] = [isize::MIN, -100_000, -1, 0, 100_000, isize::MAX];

// float は NaN / ∞ / 範囲外も含める。飽和・NaN→0 の regime でも
// `cast == as` が保たれることを確認するため（比較結果は整数なので float_cmp は起きない）。
const F32S: [f32; 15] = [
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
const F64S: [f64; 17] = [
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

// 変換先 `$t`（メソッド `$m`）について、渡された各ソース配列の全要素で
// `v.$m() == v as $t` を確認する。
macro_rules! check_to {
    ($m:ident, $t:ty; $($arr:expr),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                assert_eq!(
                    v.$m(),
                    v as $t,
                    "source {:?} -> {}",
                    v,
                    stringify!($t),
                );
            }
        )+
    };
}

/// 符号なし整数への変換が `as` と一致する（全ソース型を網羅）。
#[test]
fn as_agreement_to_unsigned() {
    check_to!(cast_u8, u8;
        U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_u16, u16;
        U8S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_u32, u32;
        U8S, U16S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_u64, u64;
        U8S, U16S, U32S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_u128, u128;
        U8S, U16S, U32S, U64S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_usize, usize;
        U8S, U16S, U32S, U64S, U128S,
        I8S, I16S, I32S, I64S, I128S, ISIZES, F32S, F64S);
}

/// 符号あり整数への変換が `as` と一致する（全ソース型を網羅）。
#[test]
fn as_agreement_to_signed() {
    check_to!(cast_i8, i8;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I16S, I32S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_i16, i16;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I32S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_i32, i32;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I64S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_i64, i64;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I128S, ISIZES, F32S, F64S);
    check_to!(cast_i128, i128;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, ISIZES, F32S, F64S);
    check_to!(cast_isize, isize;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, F32S, F64S);
}

// ========== 2. 整数 → 整数のゴールデン ==========

/// widening・同符号: 値がロスなく保たれる。
#[test]
fn int_widening_preserves_value() {
    // 符号なし: ゼロ拡張
    assert_eq!(200u8.cast_u32(), 200);
    assert_eq!(200u8.cast_u64(), 200);
    assert_eq!(65_535u16.cast_u32(), 65_535);
    // 符号あり: 符号拡張（負値も保存）
    assert_eq!((-200i16).cast_i32(), -200);
    assert_eq!((-1i8).cast_i64(), -1);
    assert_eq!(i8::MIN.cast_i32(), -128);
}

/// narrowing: 下位ビットを残す（回り込む）。
#[test]
fn int_narrowing_keeps_low_bits() {
    // 300 = 0x12C → 下位バイト 0x2C = 44
    assert_eq!(300u32.cast_u8(), 44);
    // 0xABCD → 下位バイト 0xCD = 205
    assert_eq!(0xABCDu16.cast_u8(), 205);
    // 70_000 = 0x1_1170 → 下位 16bit 0x1170 = 4464
    assert_eq!(70_000u32.cast_u16(), 4_464);
}

/// 符号またぎ・同幅: ビットの再解釈。
#[test]
fn int_sign_reinterpret_same_width() {
    // 負値 → 全ビット 1
    assert_eq!((-1i32).cast_u32(), u32::MAX);
    assert_eq!((-1i8).cast_u8(), 255);
    assert_eq!((-1i64).cast_u64(), u64::MAX);
    // 全ビット 1 → -1
    assert_eq!(u32::MAX.cast_i32(), -1);
    assert_eq!(255u8.cast_i8(), -1);
}

/// 符号あり narrowing: 負の中間値の切り詰め。
#[test]
fn int_signed_narrowing() {
    // -1000 = …0xFC18 → 下位バイト 0x18 = 24
    assert_eq!((-1000i32).cast_u8(), 24);
    assert_eq!((-1000i32).cast_i8(), 24);
    // -300 = …0xFED4 → 下位バイト 0xD4 = 212
    assert_eq!((-300i16).cast_u8(), 212);
    // -129 = …0xFF7F → 下位バイト 0x7F = 127（i8 に収まらない値の回り込み）
    assert_eq!((-129i16).cast_i8(), 127);
}

/// 境界値（MAX / MIN）を縮む方向へ変換したときの張り付き / 回り込み。
#[test]
fn int_boundary_narrowing() {
    // 符号なし MAX の下位ビット
    assert_eq!(u16::MAX.cast_u8(), 255);
    assert_eq!(u32::MAX.cast_u8(), 255);
    assert_eq!(u32::MAX.cast_u16(), 65_535);
    // 符号あり MAX/MIN の下位バイト
    assert_eq!(i32::MAX.cast_i8(), -1); // 0x7FFF_FFFF → 0xFF
    assert_eq!(i32::MIN.cast_i8(), 0); // 0x8000_0000 → 0x00
    // 符号あり境界 → 符号なし
    assert_eq!(i16::MIN.cast_u8(), 0); // 0x8000 → 0x00
    assert_eq!(i16::MAX.cast_u8(), 255); // 0x7FFF → 0xFF
    assert_eq!(u8::MAX.cast_i8(), -1); // 0xFF → -1
}

// ========== 3. 浮動小数点 → 整数のゴールデン ==========

/// 小数部はゼロ方向へ切り捨てる（floor ではない）。負値で必ず確認する。
#[test]
fn float_truncates_toward_zero() {
    assert_eq!(3.9f32.cast_i32(), 3);
    assert_eq!((-3.9f32).cast_i32(), -3); // floor なら -4。ゼロ方向なので -3
    assert_eq!(3.9f64.cast_i64(), 3);
    assert_eq!((-3.9f64).cast_i64(), -3);
}

/// 範囲外の値は変換先の MAX / MIN に飽和する。
#[test]
fn float_out_of_range_saturates() {
    assert_eq!(1e30f32.cast_i32(), i32::MAX);
    assert_eq!((-1e30f32).cast_i32(), i32::MIN);
    assert_eq!(1e300f64.cast_i64(), i64::MAX);
    assert_eq!((-1e300f64).cast_i64(), i64::MIN);
}

/// 負値 → 符号なし型は 0 に張り付く。
#[test]
fn float_negative_to_unsigned_is_zero() {
    assert_eq!((-5.0f32).cast_u8(), 0);
    assert_eq!((-1e30f32).cast_u32(), 0);
    assert_eq!((-5.0f64).cast_u8(), 0);
}

/// 大きすぎる値 → 符号なし型は MAX に飽和する。
#[test]
fn float_too_large_to_unsigned_saturates() {
    assert_eq!(300.0f32.cast_u8(), 255);
    assert_eq!(1e30f32.cast_u32(), u32::MAX);
    assert_eq!(300.0f64.cast_u8(), 255);
}

/// NaN は 0。
#[test]
fn float_nan_is_zero() {
    assert_eq!(f32::NAN.cast_i32(), 0);
    assert_eq!(f32::NAN.cast_u8(), 0);
    assert_eq!(f64::NAN.cast_i64(), 0);
}

/// 無限大は MAX / MIN に飽和する。
#[test]
fn float_infinity_saturates() {
    assert_eq!(f32::INFINITY.cast_i32(), i32::MAX);
    assert_eq!(f32::NEG_INFINITY.cast_i32(), i32::MIN);
    assert_eq!(f32::INFINITY.cast_u8(), 255);
    assert_eq!(f64::INFINITY.cast_i64(), i64::MAX);
    assert_eq!(f64::NEG_INFINITY.cast_i64(), i64::MIN);
}

/// 0.0 と -0.0 はどちらも 0。
#[test]
fn float_zero_and_neg_zero() {
    assert_eq!(0.0f32.cast_i32(), 0);
    assert_eq!((-0.0f32).cast_i32(), 0);
    assert_eq!(0.0f64.cast_i64(), 0);
    assert_eq!((-0.0f64).cast_i64(), 0);
}
