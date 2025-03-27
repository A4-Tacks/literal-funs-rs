#![no_std]
#![doc = include_str!("../README.md")]

macro_rules! gen_fun {
    ($(
        $(#[$meta:meta])*
        $name:ident [$($value:expr),* $(,)?]
    )+) => {
        $(
            $(#[$meta])*
            #[doc = concat!("Return a [`prim@", stringify!($name), "`] value\n")]
            ///
            /// # Examples
            /// ```
            #[doc = concat!("use literal_funs::", stringify!($name), ";\n")]
            #[doc = concat!(
                $(
                    "assert_eq!(",
                    stringify!($name), "::<", stringify!($value), ">(()), ",
                    stringify!($value), ");\n",
                )*
            )]
            /// ```
            ///
            /// See the [module-level documentation](./index) for more
            pub fn $name<const L: $name>(_: impl Sized) -> $name {
                L
            }
        )+
    };
}

gen_fun! {
    bool    [true, false]
    char    ['a', '\0', '你', ' ']
    i8      [-5, 0, 123]
    i16     [-5, 0, 123]
    i32     [-5, 0, 123]
    i64     [-5, 0, 123]
    i128    [-5, 0, 123]
    isize   [-5, 0, 123]
    u8      [0, 123]
    u16     [0, 123]
    u32     [0, 123]
    u64     [0, 123]
    u128    [0, 123]
    usize   [0, 123]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(bool::<true>(3), true);
        assert_eq!(bool::<false>(3), false);
        assert_eq!(i32::<8>(3), 8);
    }
}
