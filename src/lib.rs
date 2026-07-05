#![cfg_attr(
    all(feature = "cast", feature = "checked-cast"),
    doc = include_str!("../README.md")
)]
#![no_std]

// すべての処理をすべてのfeatureからアクセスするわけではないため未使用警告を抑制しておく
#[allow(unused_macros, unused_imports)]
#[cfg(any(feature = "cast", feature = "checked-cast"))]
pub(crate) mod utility;

#[cfg(feature = "cast")]
pub mod cast;

#[cfg(feature = "checked-cast")]
pub mod checked_cast;

#[cfg(test)]
mod tests {
    #[cfg(feature = "cast")]
    mod cast_float;

    #[cfg(feature = "cast")]
    mod cast_int;

    #[cfg(any(feature = "cast", feature = "checked-cast"))]
    pub(super) mod cast_utility;

    #[cfg(feature = "checked-cast")]
    mod checked_cast_float;

    #[cfg(feature = "checked-cast")]
    mod checked_cast_int;
}
