macro_rules! for_unsigned_ints {
    ($mac:ident) => {
        $mac!(u8, u16, u32, u64, u128, usize);
    };
}

macro_rules! for_signed_ints {
    ($mac:ident) => {
        $mac!(i8, i16, i32, i64, i128, isize);
    };
}

macro_rules! for_floats {
    ($mac:ident) => {
        $mac!(f32, f64);
    };
}

pub(crate) use for_floats;
pub(crate) use for_signed_ints;
pub(crate) use for_unsigned_ints;
