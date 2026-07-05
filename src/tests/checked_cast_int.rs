//! `CheckedCastU8` … `CheckedCastIsize`（整数型への変換）のテスト。
//!
//! テストは 2 層構成:
//!
//! 1. 網羅層（全 impl の生成保証）:
//!    - 整数 → 整数: `v.checked_cast_xx() == <xx>::try_from(v).ok()` を確認する。
//!      実装（`convert_int_to_int!`）自体が `try_from` への委譲なので判定基準としては
//!      同語反復だが、既存 `cast` テストと同様「マクロが全型に impl を生成したこと」の
//!      保証を目的とする。
//!    - 全ソース共通: `Some(x)` が返った場合に `x == v as xx` かつ
//!      `x as ソース型 == v`（往復無損失）を確認する。
//! 2. ゴールデン層: `Some` / `None` の境界を固定値で押さえる。
//!
//! # オラクルの使い分け原則
//!
//! round-trip 比較は「`Some` が正しい」ことの検証にのみ使える。「`None` であるべき」の
//! 判定には使えない — 飽和・∞ 経由で往復が偽一致する反例があるため
//! （例: `u64::MAX as f32` は 2^64 に丸まり、`as u64` で飽和して `u64::MAX` に戻る）。
//! `None` 側の正しさはすべてゴールデン（固定値）で固定する。
//!
//! なお恒等変換（例: `u8` の `checked_cast_u8`）は実装されていないため、網羅層では
//! 変換先と同じ型の入力は除外する。

use super::cast_utility::{
    F32S, F64S, I8S, I16S, I32S, I64S, I128S, ISIZES, U8S, U16S, U32S, U64S, U128S, USIZES,
};
use crate::checked_cast::{
    CheckedCastI8, CheckedCastI16, CheckedCastI32, CheckedCastI64, CheckedCastI128,
    CheckedCastIsize, CheckedCastU8, CheckedCastU16, CheckedCastU32, CheckedCastU64,
    CheckedCastU128, CheckedCastUsize,
};

// ========== 1. 網羅層 ==========

// 変換先 `$t`（メソッド `$m`）について、渡された各整数ソース配列の全要素で
// `v.$m() == <$t>::try_from(v).ok()` を確認する。
macro_rules! check_try_from_to {
    ($m:ident, $t:ty; $($arr:expr),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                assert_eq!(
                    v.$m(),
                    <$t>::try_from(v).ok(),
                    "source {:?} -> {}",
                    v,
                    stringify!($t),
                );
            }
        )+
    };
}

// 変換先 `$t`（メソッド `$m`）について、ソース型 `$src` の配列 `$arr` の全要素で
// 「`Some(x)` なら `x == v as $t` かつ `x as $src == v`（往復無損失）」を確認する。
// `None` 側はここでは検証しない（モジュールコメントのオラクル使い分け原則を参照）。
macro_rules! check_some_round_trip_to {
    ($m:ident, $t:ty; $($arr:expr => $src:ty),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                if let Some(x) = v.$m() {
                    assert_eq!(x, v as $t, "source {:?} -> {}", v, stringify!($t));
                    assert_eq!(
                        x as $src,
                        v,
                        "round-trip loss: {:?} -> {}",
                        v,
                        stringify!($t),
                    );
                }
            }
        )+
    };
}

/// 整数 → 符号なし整数が `try_from` と一致する（全ソース整数型を網羅）。
#[test]
fn try_from_agreement_to_unsigned() {
    check_try_from_to!(checked_cast_u8, u8;
        U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_u16, u16;
        U8S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_u32, u32;
        U8S, U16S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_u64, u64;
        U8S, U16S, U32S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_u128, u128;
        U8S, U16S, U32S, U64S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_usize, usize;
        U8S, U16S, U32S, U64S, U128S,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
}

/// 整数 → 符号あり整数が `try_from` と一致する（全ソース整数型を網羅）。
#[test]
fn try_from_agreement_to_signed() {
    check_try_from_to!(checked_cast_i8, i8;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I16S, I32S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_i16, i16;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I32S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_i32, i32;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I64S, I128S, ISIZES);
    check_try_from_to!(checked_cast_i64, i64;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I128S, ISIZES);
    check_try_from_to!(checked_cast_i128, i128;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, ISIZES);
    check_try_from_to!(checked_cast_isize, isize;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S);
}

/// Some 側の往復無損失プロパティ: 符号なし整数ターゲット（全ソース型を網羅）。
///
/// 往復の比較は実装の判定と同じ `==` で行う（`-0.0 → Some(0)` の往復
/// `0 as f32 == -0.0f32` は `to_bits` 比較では成立しないため）。
#[test]
#[allow(clippy::float_cmp)]
fn some_round_trip_to_unsigned() {
    check_some_round_trip_to!(checked_cast_u8, u8;
        U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_u16, u16;
        U8S => u8, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_u32, u32;
        U8S => u8, U16S => u16, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_u64, u64;
        U8S => u8, U16S => u16, U32S => u32, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_u128, u128;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_usize, usize;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
}

/// Some 側の往復無損失プロパティ: 符号あり整数ターゲット（全ソース型を網羅）。
///
/// 往復の比較は実装の判定と同じ `==` で行う（`-0.0 → Some(0)` の往復
/// `0 as f32 == -0.0f32` は `to_bits` 比較では成立しないため）。
#[test]
#[allow(clippy::float_cmp)]
fn some_round_trip_to_signed() {
    check_some_round_trip_to!(checked_cast_i8, i8;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_i16, i16;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_i32, i32;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_i64, i64;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I128S => i128, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_i128, i128;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, ISIZES => isize,
        F32S => f32, F64S => f64);
    check_some_round_trip_to!(checked_cast_isize, isize;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128,
        F32S => f32, F64S => f64);
}

// ========== 2. 整数 → 整数のゴールデン ==========

/// 縮小: 境界ちょうどは Some、+1 超えは None。
#[test]
fn int_narrowing_boundary() {
    assert_eq!(255u16.checked_cast_u8(), Some(255));
    assert_eq!(256u16.checked_cast_u8(), None);
    assert_eq!((-128i16).checked_cast_i8(), Some(-128));
    assert_eq!((-129i16).checked_cast_i8(), None);
}

/// 符号なし → 符号あり同幅: ターゲットの MAX を境に Some / None が分かれる。
#[test]
fn int_unsigned_to_signed_same_width() {
    assert_eq!(127u8.checked_cast_i8(), Some(127));
    assert_eq!(128u8.checked_cast_i8(), None);
}

/// 負値 → 符号なしはすべて None（`as` なら回り込む値なので、回り込み検出の証明になる）。
#[test]
fn int_negative_to_unsigned_is_none() {
    assert_eq!((-1i8).checked_cast_u8(), None);
    assert_eq!((-1i8).checked_cast_u16(), None);
    assert_eq!((-1i8).checked_cast_u32(), None);
    assert_eq!((-1i8).checked_cast_u64(), None);
    assert_eq!((-1i8).checked_cast_u128(), None);
    assert_eq!((-1i8).checked_cast_usize(), None);
}

/// 拡大変換は常に Some（値保存）。
#[test]
fn int_widening_is_some() {
    assert_eq!(u8::MAX.checked_cast_u128(), Some(255));
    assert_eq!(i8::MIN.checked_cast_i128(), Some(-128));
    assert_eq!(i8::MAX.checked_cast_i128(), Some(127));
}

/// usize / isize: ポインタ幅に依存しないケースのみ固定する
/// （`u128::MAX` はどのポインタ幅でも usize に収まらない）。
#[test]
fn int_usize_boundary() {
    assert_eq!(0usize.checked_cast_u8(), Some(0));
    assert_eq!(u128::MAX.checked_cast_usize(), None);
}

// ========== 3. 浮動小数点 → 整数のゴールデン ==========

/// 整数値ぴったりの浮動小数点は Some。
#[test]
fn float_exact_integer_is_some() {
    assert_eq!(42.0f32.checked_cast_i32(), Some(42));
    assert_eq!((-5.0f64).checked_cast_i64(), Some(-5));
}

/// 小数部があれば None（`as` は切り捨てるが checked は弾く、という差分の固定）。
#[test]
fn float_fractional_is_none() {
    assert_eq!(3.9f32.checked_cast_i32(), None);
    assert_eq!((-3.9f32).checked_cast_i32(), None);
    assert_eq!(42.5f32.checked_cast_i32(), None);
    assert_eq!(3.9f64.checked_cast_i64(), None);
    assert_eq!((-3.9f64).checked_cast_i64(), None);
    assert_eq!(42.5f64.checked_cast_i64(), None);
}

/// -0.0 は `0.0 == -0.0` により Some(0) になる（仕様の文書化）。
#[test]
fn float_negative_zero_is_some_zero() {
    assert_eq!((-0.0f32).checked_cast_u8(), Some(0));
    assert_eq!((-0.0f64).checked_cast_i64(), Some(0));
}

/// 負値 → 符号なしは None（`as` なら 0 に飽和する値）。
#[test]
fn float_negative_to_unsigned_is_none() {
    assert_eq!((-1.0f32).checked_cast_u8(), None);
}

/// 範囲外は None（`as` なら MAX に飽和する値）。
#[test]
fn float_out_of_range_is_none() {
    assert_eq!(300.0f32.checked_cast_u8(), None);
    assert_eq!(1e30f32.checked_cast_u32(), None);
}

// `$v` を全整数ターゲットへ変換し、すべて None であることを確認する。
macro_rules! check_none_for_all_int_targets {
    ($v:expr) => {{
        let v = $v;
        assert_eq!(v.checked_cast_u8(), None);
        assert_eq!(v.checked_cast_u16(), None);
        assert_eq!(v.checked_cast_u32(), None);
        assert_eq!(v.checked_cast_u64(), None);
        assert_eq!(v.checked_cast_u128(), None);
        assert_eq!(v.checked_cast_usize(), None);
        assert_eq!(v.checked_cast_i8(), None);
        assert_eq!(v.checked_cast_i16(), None);
        assert_eq!(v.checked_cast_i32(), None);
        assert_eq!(v.checked_cast_i64(), None);
        assert_eq!(v.checked_cast_i128(), None);
        assert_eq!(v.checked_cast_isize(), None);
    }};
}

/// NaN は全整数ターゲットで None（早期 return の検証）。
#[test]
fn float_nan_is_none() {
    check_none_for_all_int_targets!(f32::NAN);
    check_none_for_all_int_targets!(f64::NAN);
}

/// ±∞ は None（`as` なら MAX / MIN に飽和する値）。
#[test]
fn float_infinity_is_none() {
    assert_eq!(f32::INFINITY.checked_cast_i32(), None);
    assert_eq!(f32::NEG_INFINITY.checked_cast_i32(), None);
    assert_eq!(f32::INFINITY.checked_cast_u8(), None);
    assert_eq!(f32::NEG_INFINITY.checked_cast_u8(), None);
    assert_eq!(f64::INFINITY.checked_cast_i64(), None);
    assert_eq!(f64::NEG_INFINITY.checked_cast_i64(), None);
}

/// MAX ガードの発火（このモジュールで最重要）: 飽和した cast の round-trip が
/// 偽一致するケースを `!(cast == MAX && BITS > MANTISSA_DIGITS)` ガードが正しく弾く。
#[test]
fn float_max_guard_fires() {
    // 2^32: u32 へは飽和で u32::MAX になり、`u32::MAX as f32` も 2^32 に丸まって偽一致する
    assert_eq!(4_294_967_296.0f32.checked_cast_u32(), None);
    // 2^64: 同上（u64 版）
    assert_eq!(18_446_744_073_709_551_616.0f32.checked_cast_u64(), None);
    // ∞ は u128::MAX に飽和し、`u128::MAX as f32` も ∞ に丸まって偽一致する
    assert_eq!(f32::INFINITY.checked_cast_u128(), None);
}

/// MAX ガードの不発: ターゲット MAX ちょうどへの正当な変換が誤って弾かれないこと。
#[test]
fn float_max_guard_does_not_misfire() {
    // u16::BITS(16) ≤ f32::MANTISSA_DIGITS(24) なのでガード対象外
    assert_eq!(65_535.0f32.checked_cast_u16(), Some(u16::MAX));
    // u32::BITS(32) ≤ f64::MANTISSA_DIGITS(53)
    assert_eq!(4_294_967_295.0f64.checked_cast_u32(), Some(u32::MAX));
    // f32::MAX = (2^24 − 1) × 2^104。cast 結果が u128::MAX に一致しないためガード不発
    assert_eq!(
        f32::MAX.checked_cast_u128(),
        Some(340_282_346_638_528_859_811_704_183_484_516_925_440),
    );
    // ガードは MAX のみ見る。MIN（−2^31 / −2^63）は 2 の冪で正確に表現できるので Some
    assert_eq!((-2_147_483_648.0f32).checked_cast_i32(), Some(i32::MIN));
    assert_eq!(
        (-9_223_372_036_854_775_808.0f64).checked_cast_i64(),
        Some(i64::MIN)
    );
}

/// 2^53 を超えても f64 が実際に保持している整数値はそのまま無損失で変換できる。
#[test]
fn float_beyond_mantissa_but_exact_is_some() {
    // 1e30 の f64 近似値は整数（53bit 仮数 × 2^47）なので u128 に無損失で収まる
    assert_eq!(
        1e30f64.checked_cast_u128(),
        Some(1_000_000_000_000_000_019_884_624_838_656),
    );
}

/// usize / isize ターゲットの代表ケース。
#[test]
fn float_to_usize_isize() {
    assert_eq!(42.0f32.checked_cast_usize(), Some(42));
    assert_eq!((-1.0f32).checked_cast_usize(), None);
    assert_eq!((-5.0f64).checked_cast_isize(), Some(-5));
}
