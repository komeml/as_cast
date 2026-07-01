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

cast_f32!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64
);

cast_f64!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32
);
