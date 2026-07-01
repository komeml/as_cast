use crate::macros::{for_float, for_signed_scalars, for_unsigned_scalars};

pub trait CastF32 {
    fn cast_f32(self) -> f32;
}

pub trait CastF64 {
    fn cast_f64(self) -> f64;
}

macro_rules! cast_f32 {
    ($($type:ty), *) => {
        $(
        impl CastF32 for $type {
            fn cast_f32(self) -> f32 {
                self as f32
            }
        }
        )*
    };
}

macro_rules! cast_f64 {
    ($($type:ty), *) => {
        $(
        impl CastF64 for $type {
            fn cast_f64(self) -> f64 {
                self as f64
            }
        }
        )*
    };
}

for_unsigned_scalars!(cast_f32);
for_signed_scalars!(cast_f32);
for_float!(cast_f32);

for_unsigned_scalars!(cast_f64);
for_signed_scalars!(cast_f64);
for_float!(cast_f64);
