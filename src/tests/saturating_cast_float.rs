//! `SaturatingCastF32` / `SaturatingCastF64`（浮動小数点型への変換）のテスト。
//!
//! テストは 3 層構成:
//!
//! 1. 網羅層（全 impl の生成保証）: `v as xx` とのビット一致を確認する。
//!    `simple_as` 経路は実装がそのまま `as` なので同語反復だが、既存テストと同様
//!    「マクロが全型に impl を生成したこと」の保証を目的とする。
//!    飽和処理を持つ 2 経路は期待値の分岐を書くと実装の写しになるため網羅層から外し、
//!    すべてゴールデン層で固定する:
//!    - u128 → f32（`saturating_cast_u128_to_f32!`）→ 2. のゴールデン
//!    - f64 → f32（`saturating_cast_float_to_float!`）→ 3. のゴールデン
//! 2. / 3. ゴールデン層: 飽和が発火する / しない境界、±∞ / NaN / subnormal を
//!    固定値で押さえる。
//! 4. プロパティ層（単調性）: `v1 <= v2` なら `cast(v1) <= cast(v2)`（順序保存）。
//!
//! float 比較は ±0.0 の区別と `clippy::float_cmp` 回避のため `to_bits` で行う。
//! なお恒等変換（f32 の `saturating_cast_f32`、f64 の `saturating_cast_f64`）は
//! 実装されていないため、各層とも変換先と同じ型の入力は除外する。

use super::cast_utility::{
    F32S, F32S_ASCENDING, F64S_ASCENDING, I8S, I16S, I32S, I64S, I128S, ISIZES, U8S, U16S, U32S,
    U64S, U128_TO_F32_OVERFLOW_MIDPOINT, U128S, USIZES, check_monotonic,
};
use crate::saturating_cast::{SaturatingCastF32, SaturatingCastF64};

/// 浮動小数点を `to_bits()` で厳密比較する（±0.0 を区別し、`clippy::float_cmp` を回避）。
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

// ========== 1. 網羅層 ==========

// 変換先 `$t`（メソッド `$m`）について、渡された各ソース配列の全要素で
// `v.$m()` が `v as $t` とビット単位で一致することを確認する。
macro_rules! check_as_bits_to {
    ($m:ident, $t:ty; $($arr:expr),+ $(,)?) => {
        $(
            for &v in $arr.iter() {
                assert_bits_eq!(v.$m(), v as $t);
            }
        )+
    };
}

/// 整数 → f32 が `as` とビット一致する
/// （u128 のみ飽和処理を持つため除外し、ゴールデン層で固定する）。
#[test]
fn as_agreement_to_f32() {
    check_as_bits_to!(saturating_cast_f32, f32;
        U8S, U16S, U32S, U64S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES);
}

/// 整数・f32 → f64 が `as` とビット一致する。u128 → f64 は `simple_as` なので含める。
/// f32 → f64 の拡大は無損失で、NaN / ±∞ も両辺同じ演算のため一致する。
#[test]
fn as_agreement_to_f64() {
    check_as_bits_to!(saturating_cast_f64, f64;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S);
}

// ========== 2. u128 / i128 → float のゴールデン ==========

/// u128 → f32 の上方飽和（このモジュールの核心その 1）:
/// `as` なら ∞ に丸まる値が f32::MAX に飽和する。
#[test]
fn u128_to_f32_saturates_to_max() {
    // 前提の確認: `as` 自体は ∞ に丸める
    assert!((u128::MAX as f32).is_infinite());
    assert_bits_eq!(u128::MAX.saturating_cast_f32(), f32::MAX);
}

/// u128 → f32 の飽和しない側: 有限に収まる値は `as` と一致する。
#[test]
fn u128_to_f32_finite_matches_as() {
    assert_bits_eq!(0u128.saturating_cast_f32(), 0.0f32);
    // 2^127 は f32 で正確に表現できる（有限）
    let v = 1u128 << 127;
    assert!(v.saturating_cast_f32().is_finite());
    assert_bits_eq!(v.saturating_cast_f32(), v as f32);
    // f32::MAX（2^128 − 2^104）は u128 に正確に収まり、正確に往復する
    assert_bits_eq!((f32::MAX as u128).saturating_cast_f32(), f32::MAX);
}

/// u128 → f32 の丸め境界（2^128 − 2^103）: 境界未満は直接 f32::MAX へ丸められ、
/// 境界以上は ∞ に丸まってから飽和で f32::MAX に戻る。
/// 同じ結果に異なる経路で到達することの固定。
#[test]
fn u128_to_f32_rounding_boundary() {
    // 前提の確認: 境界の前後で `as` の丸め先が変わる
    assert!(((U128_TO_F32_OVERFLOW_MIDPOINT - 1) as f32).is_finite());
    assert!((U128_TO_F32_OVERFLOW_MIDPOINT as f32).is_infinite());
    // 直接丸め（飽和不発）
    assert_bits_eq!(
        (U128_TO_F32_OVERFLOW_MIDPOINT - 1).saturating_cast_f32(),
        f32::MAX
    );
    // ∞ → 飽和経由
    assert_bits_eq!(
        U128_TO_F32_OVERFLOW_MIDPOINT.saturating_cast_f32(),
        f32::MAX
    );
}

/// u128 → f64 は飽和不要: u128::MAX（≈ 3.4e38）は f64 の範囲内で有限。
/// 「u128 → f32 だけが特殊」という設計判断の固定。
#[test]
fn u128_to_f64_needs_no_saturation() {
    let x = u128::MAX.saturating_cast_f64();
    assert!(x.is_finite());
    assert_bits_eq!(x, u128::MAX as f64);
}

/// i128 → f32 も飽和不要: i128 の絶対値（≤ 2^127）は f32::MAX を超えないため
/// `simple_as` で正しい、という理由の文書化。
#[test]
fn i128_to_f32_needs_no_saturation() {
    // i128::MIN = -2^127 は f32 で正確に表現できる（有限）
    let min = i128::MIN.saturating_cast_f32();
    assert!(min.is_finite());
    assert_bits_eq!(min, i128::MIN as f32);
    let max = i128::MAX.saturating_cast_f32();
    assert!(max.is_finite());
    assert_bits_eq!(max, i128::MAX as f32);
}

/// 精度損失は許容（`as` 準拠）: checked_cast なら None になる値が黙って丸まる。
#[test]
fn int_to_float_precision_loss_follows_as() {
    // u64::MAX は f32 で 2^64 に丸まる
    assert_bits_eq!(
        u64::MAX.saturating_cast_f32(),
        18_446_744_073_709_551_616.0f32
    );
    // 2^24 + 1 は f32 で表現できず 2^24 に丸まる
    assert_bits_eq!(((1u32 << 24) + 1).saturating_cast_f32(), 16_777_216.0f32);
}

// ========== 3. f64 → f32 のゴールデン ==========

/// 範囲内・正確に表現できる値は飽和せずそのまま（境界ちょうどの f32::MAX を含む）。
#[test]
fn f64_to_f32_in_range_exact() {
    assert_bits_eq!(1.5f64.saturating_cast_f32(), 1.5f32);
    assert_bits_eq!((f32::MAX as f64).saturating_cast_f32(), f32::MAX);
}

/// 範囲内・丸めは `as` 準拠。
#[test]
fn f64_to_f32_rounding_follows_as() {
    assert_bits_eq!(0.1f64.saturating_cast_f32(), 0.1f64 as f32);
}

/// 上方飽和: `as` なら ∞ に丸まる有限値が f32::MAX になる。
#[test]
fn f64_to_f32_saturates_to_max() {
    assert_bits_eq!(f64::MAX.saturating_cast_f32(), f32::MAX);
    assert_bits_eq!(1e300f64.saturating_cast_f32(), f32::MAX);
}

/// 下方飽和: `as` なら −∞ に丸まる有限値が f32::MIN になる
/// （u128 版で負の無限大の考慮漏れがあった経緯から、負側分岐を重点的に固定する）。
#[test]
fn f64_to_f32_saturates_to_min() {
    assert_bits_eq!(f64::MIN.saturating_cast_f32(), f32::MIN);
    assert_bits_eq!((-1e300f64).saturating_cast_f32(), f32::MIN);
}

/// ±∞ は飽和させず保存する（最重要）: `$v.is_finite()` ガードの存在理由そのもので、
/// これが無いと ∞ が MAX に化ける。
#[test]
fn f64_to_f32_infinity_is_preserved() {
    assert_bits_eq!(f64::INFINITY.saturating_cast_f32(), f32::INFINITY);
    assert_bits_eq!(f64::NEG_INFINITY.saturating_cast_f32(), f32::NEG_INFINITY);
}

/// NaN は NaN のまま（NaN は ∞ でないため飽和分岐を素通りする経路）。
#[test]
fn f64_to_f32_nan_stays_nan() {
    assert!(f64::NAN.saturating_cast_f32().is_nan());
}

/// 飽和境界ぎわ: f32::MAX 直上の f64 は丸めで f32::MAX に戻り（飽和不発）、
/// 丸め境界（2^128 − 2^103、u128 → f32 と同じ定数）からは ∞ 経由で飽和が発火する。
#[test]
fn f64_to_f32_saturation_boundary() {
    // f32::MAX の f64 表現の nextup → 丸めで f32::MAX に戻る（飽和不発）
    let nextup = f64::from_bits((f32::MAX as f64).to_bits() + 1);
    assert_bits_eq!(nextup.saturating_cast_f32(), f32::MAX);
    // 2^128 − 2^103 は有効ビット幅 25 なので f64 で正確に表現できる
    let midpoint = U128_TO_F32_OVERFLOW_MIDPOINT as f64;
    // 前提の確認: `as` は ∞ に丸める
    assert!((midpoint as f32).is_infinite());
    assert_bits_eq!(midpoint.saturating_cast_f32(), f32::MAX);
}

/// アンダーフロー: 0 への丸めは飽和対象外（`as` 準拠）であることの文書化。
#[test]
fn f64_to_f32_underflow_follows_as() {
    assert_bits_eq!(1e-300f64.saturating_cast_f32(), 0.0f32);
    // 符号ビットは保存される
    assert_bits_eq!((-1e-300f64).saturating_cast_f32(), -0.0f32);
}

/// -0.0 は符号ビットを保って -0.0 のまま。
#[test]
fn f64_to_f32_negative_zero_keeps_sign_bit() {
    assert_bits_eq!((-0.0f64).saturating_cast_f32(), -0.0f32);
}

/// f32 の最小 subnormal に相当する値は正確に往復する。
#[test]
fn f64_to_f32_subnormal_round_trips() {
    let min_subnormal = f32::from_bits(1);
    assert_bits_eq!((min_subnormal as f64).saturating_cast_f32(), min_subnormal);
}

// ========== 4. プロパティ層（単調性） ==========

/// 単調性（順序保存）: 同一ソース配列の隣接ペア `v1 <= v2` に対して
/// `cast(v1) <= cast(v2)`。飽和・丸めのどちらの経路でも順序は保存される。
#[test]
fn monotonicity_to_floats() {
    check_monotonic!(saturating_cast_f32;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F64S_ASCENDING);
    check_monotonic!(saturating_cast_f64;
        U8S, U16S, U32S, U64S, U128S, USIZES,
        I8S, I16S, I32S, I64S, I128S, ISIZES,
        F32S_ASCENDING);
}
