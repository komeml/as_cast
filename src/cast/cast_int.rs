pub trait CastU8 {
    fn cast_u8(self) -> u8;
}

pub trait CastU16 {
    fn cast_u16(self) -> u16;
}

pub trait CastU32 {
    fn cast_u32(self) -> u32;
}

pub trait CastU64 {
    fn cast_u64(self) -> u64;
}

pub trait CastU128 {
    fn cast_u128(self) -> u128;
}

pub trait CastUsize {
    fn cast_usize(self) -> usize;
}

pub trait CastI8 {
    fn cast_i8(self) -> i8;
}

pub trait CastI16 {
    fn cast_i16(self) -> i16;
}

pub trait CastI32 {
    fn cast_i32(self) -> i32;
}

pub trait CastI64 {
    fn cast_i64(self) -> i64;
}

pub trait CastI128 {
    fn cast_i128(self) -> i128;
}

pub trait CastIsize {
    fn cast_isize(self) -> isize;
}

macro_rules! cast_u8 {
    ($($type:ty), *) => {
        $(
        impl CastU8 for $type {
            fn cast_u8(self) -> u8 {
                self as u8
            }
        }
        )*
    };
}

macro_rules! cast_u16 {
    ($($type:ty), *) => {
        $(
        impl CastU16 for $type {
            fn cast_u16(self) -> u16 {
                self as u16
            }
        }
        )*
    };
}

macro_rules! cast_u32 {
    ($($type:ty), *) => {
        $(
        impl CastU32 for $type {
            fn cast_u32(self) -> u32 {
                self as u32
            }
        }
        )*
    };
}

macro_rules! cast_u64 {
    ($($type:ty), *) => {
        $(
        impl CastU64 for $type {
            fn cast_u64(self) -> u64 {
                self as u64
            }
        }
        )*
    };
}

macro_rules! cast_u128 {
    ($($type:ty), *) => {
        $(
        impl CastU128 for $type {
            fn cast_u128(self) -> u128 {
                self as u128
            }
        }
        )*
    };
}

macro_rules! cast_usize {
    ($($type:ty), *) => {
        $(
        impl CastUsize for $type {
            fn cast_usize(self) -> usize {
                self as usize
            }
        }
        )*
    };
}

macro_rules! cast_i8 {
    ($($type:ty), *) => {
        $(
        impl CastI8 for $type {
            fn cast_i8(self) -> i8 {
                self as i8
            }
        }
        )*
    };
}

macro_rules! cast_i16 {
    ($($type:ty), *) => {
        $(
        impl CastI16 for $type {
            fn cast_i16(self) -> i16 {
                self as i16
            }
        }
        )*
    };
}

macro_rules! cast_i32 {
    ($($type:ty), *) => {
        $(
        impl CastI32 for $type {
            fn cast_i32(self) -> i32 {
                self as i32
            }
        }
        )*
    };
}

macro_rules! cast_i64 {
    ($($type:ty), *) => {
        $(
        impl CastI64 for $type {
            fn cast_i64(self) -> i64 {
                self as i64
            }
        }
        )*
    };
}

macro_rules! cast_i128 {
    ($($type:ty), *) => {
        $(
        impl CastI128 for $type {
            fn cast_i128(self) -> i128 {
                self as i128
            }
        }
        )*
    };
}

macro_rules! cast_isize {
    ($($type:ty), *) => {
        $(
        impl CastIsize for $type {
            fn cast_isize(self) -> isize {
                self as isize
            }
        }
        )*
    };
}

cast_u8!(
    u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

cast_u16!(
    u8, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

cast_u32!(
    u8, u16, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

cast_u64!(
    u8, u16, u32, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

cast_u128!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

cast_usize!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, isize, f32, f64
);

cast_i8!(
    u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64
);

cast_i16!(
    u8, u16, u32, u64, u128, usize, i8, i32, i64, i128, isize, f32, f64
);

cast_i32!(
    u8, u16, u32, u64, u128, usize, i8, i16, i64, i128, isize, f32, f64
);

cast_i64!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i128, isize, f32, f64
);

cast_i128!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, f32, f64
);

cast_isize!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64
);
