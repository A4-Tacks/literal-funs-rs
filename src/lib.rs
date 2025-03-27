#![no_std]
#![doc = include_str!("../README.md")]

macro_rules! gen_fun {
    ($(
        $(#[$meta:meta])*
        $name:ident;
    )+) => {
        $(
            $(#[$meta])*
            #[doc = concat!("Return a [`", stringify!($name), "`] value\n")]
            ///
            /// See the [module-level documentation](./index) for more
            pub fn $name<const L: $name>(_: impl Sized) -> $name {
                L
            }
        )+
    };
}

gen_fun! {
    bool;
    char;
    i8;
    i16;
    i32;
    i64;
    i128;
    isize;
    u8;
    u16;
    u32;
    u64;
    u128;
    usize;
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
