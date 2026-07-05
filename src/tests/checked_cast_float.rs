//! `CheckedCastF32` / `CheckedCastF64`（浮動小数点型への変換）のテスト。
//!
//! テストは 2 層構成:
//!
//! 1. 網羅層（全 impl の生成保証）: `Some(x)` が返った場合に `x` が `v as xx` と
//!    ビット単位で一致し、`x as ソース型 == v`（往復無損失）であることを確認する。
//! 2. ゴールデン層: 仮数部境界・有効ビット幅・NaN / ∞ / subnormal といった
//!    `Some` / `None` の境界を固定値で押さえる。
//!
//! # オラクルの使い分け原則
//!
//! round-trip 比較は「`Some` が正しい」ことの検証にのみ使える。「`None` であるべき」の
//! 判定には使えない — 飽和・∞ 経由で往復が偽一致する反例があるため
//! （例: `u64::MAX as f32` は 2^64 に丸まり、`as u64` で飽和して `u64::MAX` に戻る）。
//! `None` 側の正しさはすべてゴールデン（固定値）で固定する。
//!
//! なお恒等変換（f32 の `checked_cast_f32`、f64 の `checked_cast_f64`）は実装されて
//! いないため、網羅層では変換先と同じ型の入力は除外する。

use super::cast_utility::{
    F32_MANTISSA_LIMIT, F32_MANTISSA_LIMIT_MINUS_1, F32_MANTISSA_LIMIT_PLUS_1, F32S,
    F64_MANTISSA_LIMIT, F64_MANTISSA_LIMIT_MINUS_1, F64_MANTISSA_LIMIT_PLUS_1, F64S, I8S, I16S,
    I32S, I64S, I128S, ISIZES, U8S, U16S, U32S, U64S, U128S, USIZES,
};
use crate::checked_cast::{CheckedCastF32, CheckedCastF64};

// ========== 1. 網羅層 ==========

// 変換先 `$t`（メソッド `$m`）について、ソース型 `$src` の配列 `$arr` の全要素で
// 「`Some(x)` なら `x` が `v as $t` とビット単位で一致し、`x as $src == v`
// （往復無損失）」を確認する。
// `None` 側はここでは検証しない（モジュールコメントのオラクル使い分け原則を参照）。
macro_rules! check_some_round_trip_to {
    ($m:ident, $t:ty; $($arr:expr => $src:ty),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                if let Some(x) = v.$m() {
                    // Some の値は `as` と同じキャスト結果（±0.0 も区別するビット比較）
                    assert_eq!(
                        x.to_bits(),
                        (v as $t).to_bits(),
                        "source {:?} -> {}",
                        v,
                        stringify!($t),
                    );
                    // 往復無損失。実装の判定と同じ `==` で比較する
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

/// Some 側の往復無損失プロパティ（全ソース型を網羅）。
#[test]
#[allow(clippy::float_cmp)]
fn some_round_trip_to_floats() {
    check_some_round_trip_to!(checked_cast_f32, f32;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F64S => f64);
    check_some_round_trip_to!(checked_cast_f64, f64;
        U8S => u8, U16S => u16, U32S => u32, U64S => u64, U128S => u128, USIZES => usize,
        I8S => i8, I16S => i16, I32S => i32, I64S => i64, I128S => i128, ISIZES => isize,
        F32S => f32);
}

// ========== 2. 整数 → 浮動小数点のゴールデン ==========

// 渡された各配列の全要素で `v.$m()` が Some になることを確認する。
macro_rules! check_all_some {
    ($m:ident; $($arr:expr),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                assert!(v.$m().is_some(), "expected Some: {:?}", v);
            }
        )+
    };
}

/// 仮数部に常に収まる整数型はすべて Some（値セット全走査）。
#[test]
fn int_small_types_are_always_some() {
    // u8 / i8 / u16 / i16 → f32: 有効ビット幅は最大でも 16 ≤ 24
    check_all_some!(checked_cast_f32; U8S, I8S, U16S, I16S);
    // u32 / i32 → f64: 最大でも 32 ≤ 53
    check_all_some!(checked_cast_f64; U32S, I32S);
}

/// ゼロの早期分岐: 0 は常に Some(0.0)。
#[test]
fn int_zero_is_some() {
    assert_eq!(0u8.checked_cast_f32(), Some(0.0));
    assert_eq!(0u128.checked_cast_f32(), Some(0.0));
    assert_eq!(0isize.checked_cast_f32(), Some(0.0));
    assert_eq!(0i64.checked_cast_f64(), Some(0.0));
    assert_eq!(0usize.checked_cast_f64(), Some(0.0));
}

/// f32 の仮数部境界（2^24 ± 1）。
#[test]
fn int_f32_mantissa_boundary() {
    assert_eq!(
        F32_MANTISSA_LIMIT_MINUS_1.checked_cast_f32(),
        Some(16_777_215.0),
    );
    assert_eq!(F32_MANTISSA_LIMIT.checked_cast_f32(), Some(16_777_216.0));
    assert_eq!(F32_MANTISSA_LIMIT_PLUS_1.checked_cast_f32(), None);
}

/// f64 の仮数部境界（2^53 ± 1）。
#[test]
fn int_f64_mantissa_boundary() {
    assert_eq!(
        F64_MANTISSA_LIMIT_MINUS_1.checked_cast_f64(),
        Some(9_007_199_254_740_991.0),
    );
    assert_eq!(
        F64_MANTISSA_LIMIT.checked_cast_f64(),
        Some(9_007_199_254_740_992.0),
    );
    assert_eq!(F64_MANTISSA_LIMIT_PLUS_1.checked_cast_f64(), None);
}

/// 有効ビット幅（最上位〜最下位の set bit 間の距離）による判定。
/// 値の大きさではなく「仮数部に収まるビット幅か」で決まることを固定する。
#[test]
fn int_effective_bit_width() {
    // 2^100: 有効ビット幅 1 → Some
    let v = 1u128 << 100;
    assert_eq!(v.checked_cast_f32(), Some(v as f32));
    // 2^100 + 1: 幅 101 > 24 → None
    assert_eq!(((1u128 << 100) + 1).checked_cast_f32(), None);
    // 2^100 + 2^80: 幅 21 ≤ 24 → Some
    let v = (1u128 << 100) + (1u128 << 80);
    assert_eq!(v.checked_cast_f32(), Some(v as f32));
}

/// 符号あり整数は `unsigned_abs` 経由で判定される。
#[test]
fn int_signed_path() {
    // abs = 2^24 − 1: 幅 24 ≤ 24 → Some
    assert_eq!((-16_777_215i32).checked_cast_f32(), Some(-16_777_215.0));
    // i64::MIN: abs = 2^63 で幅 1 → Some
    assert_eq!(i64::MIN.checked_cast_f64(), Some(i64::MIN as f64));
    // i64::MIN + 1: abs = 2^63 − 1 で幅 63 > 53 → None
    assert_eq!((i64::MIN + 1).checked_cast_f64(), None);
}

/// `unsigned_abs` のオーバーフロー安全性: abs（2^127）が i128 に収まらない
/// i128::MIN でもパニックせず正しく判定できる。
#[test]
fn int_i128_min_is_some() {
    assert_eq!(i128::MIN.checked_cast_f32(), Some(i128::MIN as f32));
}

/// round-trip オラクルでは検出できない偽陽性: `as` の往復は飽和 / ∞ 経由で
/// 偽一致するが、有効ビット幅判定で正しく None になる。
#[test]
fn int_round_trip_false_positive_is_none() {
    // u64::MAX as f32 は 2^64 に丸まり、as u64 で飽和して u64::MAX に戻ってしまう
    assert_eq!(u64::MAX.checked_cast_f32(), None);
    // u128::MAX as f32 は ∞ に丸まり、as u128 で飽和して u128::MAX に戻ってしまう
    assert_eq!(u128::MAX.checked_cast_f32(), None);
}

// ========== 3. 浮動小数点間のゴールデン ==========

/// f32 → f64（拡大）: 有限値は常に Some。
#[test]
fn f32_to_f64_finite_is_some() {
    assert_eq!(0.1f32.checked_cast_f64(), Some(0.1f32 as f64));
    assert_eq!(1.5f32.checked_cast_f64(), Some(1.5));
    assert_eq!(f32::MAX.checked_cast_f64(), Some(f32::MAX as f64));
}

/// f32 → f64: ±∞ は保存される。
#[test]
fn f32_to_f64_infinity_is_some() {
    assert_eq!(f32::INFINITY.checked_cast_f64(), Some(f64::INFINITY));
    assert_eq!(
        f32::NEG_INFINITY.checked_cast_f64(),
        Some(f64::NEG_INFINITY)
    );
}

/// 拡大でも NaN は None（「widening なら安全」と思い込んだ改変への防波堤）。
#[test]
fn f32_to_f64_nan_is_none() {
    assert_eq!(f32::NAN.checked_cast_f64(), None);
}

/// f64 → f32（縮小）: 正確に表現できる値は Some。
#[test]
fn f64_to_f32_exact_is_some() {
    assert_eq!(1.5f64.checked_cast_f32(), Some(1.5));
    assert_eq!((f32::MAX as f64).checked_cast_f32(), Some(f32::MAX));
    // 2 の冪は指数部だけで表現できるので正確
    assert_eq!(1024.0f64.checked_cast_f32(), Some(1024.0));
}

/// f64 → f32: 丸めでビットが変わる値・範囲外は None。
#[test]
fn f64_to_f32_lossy_is_none() {
    // f32 に丸めると値が変わる
    assert_eq!(0.1f64.checked_cast_f32(), None);
    // ∞ に丸まる
    assert_eq!(f64::MAX.checked_cast_f32(), None);
    assert_eq!(1e300f64.checked_cast_f32(), None);
    // 0 に丸まる（アンダーフロー）
    assert_eq!(1e-300f64.checked_cast_f32(), None);
}

/// f64 → f32: ±∞ は損失なし扱いで Some。
#[test]
fn f64_to_f32_infinity_is_some() {
    assert_eq!(f64::INFINITY.checked_cast_f32(), Some(f32::INFINITY));
    assert_eq!(
        f64::NEG_INFINITY.checked_cast_f32(),
        Some(f32::NEG_INFINITY)
    );
}

/// f64 → f32: f32 の最小 subnormal に相当する値は正確に戻せるので Some。
#[test]
fn f64_to_f32_subnormal_is_some() {
    let min_subnormal = f32::from_bits(1);
    assert_eq!(
        (min_subnormal as f64).checked_cast_f32(),
        Some(min_subnormal)
    );
}

/// f64 → f32: -0.0 は符号ビットを保って Some(-0.0)。
/// `==` では +0.0 と区別できないため `to_bits` で確認する。
#[test]
fn f64_to_f32_negative_zero_keeps_sign_bit() {
    let x = (-0.0f64).checked_cast_f32();
    assert_eq!(x.map(f32::to_bits), Some((-0.0f32).to_bits()));
}

/// f64 → f32 でも NaN は None。
#[test]
fn f64_to_f32_nan_is_none() {
    assert_eq!(f64::NAN.checked_cast_f32(), None);
}
