#![no_std]

#[macro_use]
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
}
