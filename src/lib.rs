#![no_std]

pub mod cast;

#[cfg(test)]
mod tests {
    mod cast_float;
    mod cast_int;
    pub(super) mod cast_utility;
}
