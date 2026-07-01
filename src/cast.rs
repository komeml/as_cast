use crate::macros::{for_floats, for_signed_ints, for_unsigned_ints};

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

for_unsigned_ints!(cast_f32);
for_signed_ints!(cast_f32);
for_floats!(cast_f32);

for_unsigned_ints!(cast_f64);
for_signed_ints!(cast_f64);
for_floats!(cast_f64);
