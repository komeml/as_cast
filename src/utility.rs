/// `as` を使ったシンプルキャスト
macro_rules! simple_as {
    ($v:ident, $t:ty) => {
        $v as $t
    };
}

/// 符号付き整数を損失なく浮動小数点数に変換できるかどうかを判定するマクロです。
macro_rules! can_convert_signed_int_to_float {
    ($v:ident, $target:ty, $src:ty) => {{
        let abs = $v.unsigned_abs();
        can_convert_unsigned_int_to_float!(abs, $target, $src)
    }};
}

/// 符号なし整数が損失なく浮動小数点数に変換可能かどうかを判定するマクロです。
macro_rules! can_convert_unsigned_int_to_float {
    ($v:ident, $target:ty, $src:ty) => {
        $v == 0
            || (<$src>::BITS - $v.leading_zeros() - $v.trailing_zeros())
                <= <$target>::MANTISSA_DIGITS
    };
}

/// 浮動小数点から整数へ損失無く変換するマクロ
///
/// 損失が発生する場合はNoneが返ってくる
macro_rules! convert_float_to_int {
    ($v:ident, $target:ty, $src:ty) => {{
        if $v.is_nan() {
            return None;
        }

        let cast = $v as $target;
        if (cast as $src) == $v
            && !(cast == <$target>::MAX && <$target>::BITS > <$src>::MANTISSA_DIGITS)
        {
            Some(cast)
        } else {
            None
        }
    }};
}

/// 整数から整数へ損失無く変換するマクロ
///
/// 損失が発生する場合はNoneが返ってくる
macro_rules! convert_int_to_int {
    ($v:ident, $target:ty) => {{ <$target>::try_from($v).ok() }};
}

/// 浮動小数点から浮動小数点へ損失無く変換するマクロ
///
/// 損失が発生する場合はNoneが返ってくる
macro_rules! convert_float_to_float {
    ($v:ident, $target:ty, $src:ty) => {{
        if $v.is_nan() {
            return None;
        }

        let cast = $v as $target;
        if (cast as $src) == $v {
            Some(cast)
        } else {
            None
        }
    }};
}

/// [`can_convert_int_to_float!`]ディスパッチマクロを生成します。
macro_rules! generate_can_convert_macro {
    ($d:tt; $($st:tt),+ $(,)? ; $($ut:tt),+ $(,)?) => {
        macro_rules! can_convert_int_to_float {
            $(
                ($d v:ident, $d target:ty, $st) => {
                    can_convert_signed_int_to_float!($d v, $d target, $st)
                };
            )+

            $(
                ($d v:ident, $d target:ty, $ut) => {
                    can_convert_unsigned_int_to_float!($d v, $d target, $ut)
                };
            )+
        }
    };
}

generate_can_convert_macro!($; i8, i16, i32, i64, i128, isize ; u8, u16, u32, u64, u128, usize);

pub(crate) use can_convert_signed_int_to_float;
pub(crate) use can_convert_unsigned_int_to_float;
pub(crate) use convert_float_to_float;
pub(crate) use convert_float_to_int;
pub(crate) use convert_int_to_int;
pub(crate) use simple_as;

// これを削除するとエラーになるが何故かClippyで警告が出るためallowで抑制しています
// Clippy側の不具合らしいので修正待ち
#[allow(clippy::single_component_path_imports)]
pub(crate) use can_convert_int_to_float;
