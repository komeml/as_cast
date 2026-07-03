#![no_std]

#[cfg(feature = "cast")]
pub mod cast;

#[cfg(test)]
mod tests {
    #[cfg(feature = "cast")]
    mod cast_float;

    #[cfg(feature = "cast")]
    mod cast_int;

    pub(super) mod cast_utility;
}
