# as_cast
[![crates.io](https://img.shields.io/crates/v/as_cast.svg)](https://crates.io/crates/as_cast)
[![docs](https://docs.rs/as_cast/badge.svg)](https://docs.rs/as_cast)
[![Rust](https://img.shields.io/badge/rust-1.85.0%2B-blue.svg?maxAge=3600)](https://github.com/komeml/as_cast)

\[[English](README.md)\]

## 概要
Rust標準の `self as T` によるキャストは、記述が増えるとコードが読みづらくなるという問題があります。

`as_cast` は、この問題を解決するために `cast_i32()` や `cast_f64()` といった**メソッド形式**でキャストを行えるようにしたクレートです。

また、キャスト時の損失を考慮した `checked_cast_i32()` や `checked_cast_f64()` といった機能も利用できます。

## 機能
各機能は feature フラグ(`cast` / `checked-cast` / `saturating-cast`)で個別に有効化できます(デフォルトはすべて有効です)。

### Cast*

標準の `as` キャストをメソッド形式で行える trait 群です。`cast_u8()` から `cast_f64()` まで、すべての数値型(`u8`〜`u128`, `usize`, `i8`〜`i128`, `isize`, `f32`, `f64`)間の相互変換に対応しています。

挙動は `as` と完全に同じであるため、**損失が発生しても検出されません**。損失を検出したい場合は `CheckedCast*` を使用してください。

```rust
use as_cast::cast::{CastU8, CastF64};

let n: i32 = 300;

assert_eq!(n.cast_u8(), 44); // `as` と同じ挙動で切り捨てられる
assert_eq!(n.cast_f64(), 300.0);
```

### CheckedCast*

変換時の損失を検出できる trait 群です。`checked_cast_*()` は `Option<T>` を返し、**損失なく変換できた場合のみ** `Some` を返します。

次の場合は `None` が返ります。

- 変換先の型の範囲外の値(例: `300i32` → `u8`)
- 浮動小数点 → 整数で小数部が失われる場合(例: `1.5f64` → `u8`)
- 整数 → 浮動小数点で仮数部の精度を超える場合(例: `16_777_217i32` → `f32`)
- `NaN` のキャスト

```rust
use as_cast::checked_cast::CheckedCastU8;

let n: i32 = 200;
assert_eq!(n.checked_cast_u8(), Some(200));

let m: i32 = 300;
assert_eq!(m.checked_cast_u8(), None); // u8 の範囲外

let f: f64 = 1.5;
assert_eq!(f.checked_cast_u8(), None); // 小数部が失われる
```

### SaturatingCast*

基本的には `Cast*` と同じ挙動でキャストする trait 群ですが、変換先の型の範囲外の値は型の最小値/最大値に丸め(飽和させ)ます。

次のように飽和されます。

- 変換先の型の範囲外の値 → 型の最小値/最大値(例: `300i32` → `u8` は `255`)
- 浮動小数点への変換で無限大になる値 → `f32::MIN`/`f32::MAX`(例: `f64::MAX` → `f32` は `f32::MAX`)
- `±∞` → 整数へは型の最小値/最大値、浮動小数点へはそのまま `±∞`
- `NaN` → 整数へは `0`、浮動小数点へはそのまま `NaN`

なお、小数部の切り捨てや仮数部の精度の損失は `as` と同様に検出されません。損失を検出したい場合は `CheckedCast*` を使用してください。

```rust
use as_cast::saturating_cast::{SaturatingCastU8, SaturatingCastF32};

let n: i32 = 300;
assert_eq!(n.saturating_cast_u8(), 255); // u8::MAX に飽和

let m: i32 = -1;
assert_eq!(m.saturating_cast_u8(), 0); // u8::MIN に飽和

let f: f64 = f64::MAX;
assert_eq!(f.saturating_cast_f32(), f32::MAX); // 無限大にならず f32::MAX に飽和
```

## 実装予定
- [x] `Cast*` : `as` と同じ挙動でキャストする
- [x] `CheckedCast*` : 損失が発生する場合は `None` を返す
- [x] `SaturatingCast*` : 範囲外の値は型の最小値/最大値に丸める
- [ ] `OverflowingCast*` : `(WrappingCast と同じ値, オーバーフローしたかどうか)` のタプルを返す
- [ ] `UnwrappedCast*` : 損失が発生する場合は panic する
