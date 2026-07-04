macro_rules! simple_as {
    ($v:ident, $t:ty) => {
        $v as $t
    };
}

macro_rules! can_convert_signed_int_to_float {
    ($v:ident, $target:ty, $src:ty) => {{
        let abs = $v.unsigned_abs();
        can_convert_unsigned_int_to_float!(abs, $target, $src)
    }};
}

macro_rules! can_convert_unsigned_int_to_float {
    ($v:ident, $target:ty, $src:ty) => {
        $v == 0
            || (<$src>::BITS - $v.leading_zeros() - $v.trailing_zeros())
                <= <$target>::MANTISSA_DIGITS
    };
}

macro_rules! generate_can_convert_macro {
    ($d:tt; $($st:tt),+ $(,)? ; $($ut:tt),+ $(,)?) => {
        macro_rules! can_convert_int_to_float {
            $(
                ($d v:ident, $d target:ty, $st) => {
                    can_convert_signed_int_to_float!($v, $target, $st)
                };
            )+

            $(
                ($d v:ident, $d target:ty, $ut) => {
                    can_convert_unsigned_int_to_float!($v, $target, $ut)
                };
            )+
        }
    };
}

generate_can_convert_macro!($; i8, i16, i32, i64, i128, isize ; u8, u16, u32, u64, u128, usize);

pub(crate) use can_convert_int_to_float;
pub(crate) use can_convert_signed_int_to_float;
pub(crate) use can_convert_unsigned_int_to_float;
pub(crate) use simple_as;
