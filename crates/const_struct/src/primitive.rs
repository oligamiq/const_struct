use core::mem::transmute;
use paste::paste;

macro_rules! PrimTraitBySizes {
    ($size:literal, $($name:ident),*) => {
        $(
            paste! {
                PrimTraitBySizes!([<u $size>], $name);
            }
        )*
    };
    ($base:ident, $($name:ident),*) => {
        $(
            paste! {
                pub trait [<$name:camel Ty>] {
                    const __DATA: $name;
                    const VALUE: $name = Self::__DATA;
                }

                pub struct [<$name:camel>]<const T: $base>;

                impl<const T: $base> [<$name:camel Ty>] for [<$name:camel>]<T> {
                    const __DATA: $name = unsafe { transmute(T) };
                }

                #[macro_export]
                macro_rules! [<$name:camel>] {
                    ($value:expr) => {
                        [<$name:camel>]::<{ unsafe { core::mem::transmute::<$name, $base>(($value)) } }>
                    };
                }
            }
        )*
    };
}

PrimTraitBySizes!(8, u8, i8, bool);
PrimTraitBySizes!(16, u16, i16);
PrimTraitBySizes!(32, f32, u32, i32, char);
PrimTraitBySizes!(64, f64, u64, i64);
PrimTraitBySizes!(128, u128, i128);
PrimTraitBySizes!(usize, usize, isize);

pub const fn tester_inner<T: F32Ty>() -> f32 {
    T::VALUE
}

pub const fn tester_inner_u32<T: U32Ty>() -> u32 {
    T::VALUE
}

#[test]
pub fn call_tester() {
    let s = F32!(-0.5);
    debug_assert_eq!(core::mem::size_of_val(&s), 0);
    debug_assert_eq!(tester_inner::<F32!(-0.5)>(), -0.5);
    debug_assert_eq!(tester_inner::<F32!(-25.333)>(), -25.333);
}
