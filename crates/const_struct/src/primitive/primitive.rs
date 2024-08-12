use crate::ConstStructTraits;
use core::mem::transmute;
use paste::paste;

pub trait PrimitiveTraits {
    type DATATYPE;
    const __DATA: Self::DATATYPE;
}

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

                pub struct [<$name:camel Impl>]<const T: $base>;

                impl<const T: $base> [<$name:camel Ty>] for [<$name:camel Impl>]<T> {
                    const __DATA: $name = unsafe { transmute(T) };
                }

                impl<U: [<$name:camel Ty>], const T: $base> ConstStructTraits<[<$name:camel Impl>]<T>> for U {
                    const __DATA: [<$name:camel Impl>]<T> = [<$name:camel Impl>]::<T>;
                }

                impl<const T: $base> PrimitiveTraits for [<$name:camel Impl>]<T> {
                    type DATATYPE = $name;
                    const __DATA: Self::DATATYPE = <[<$name:camel Impl>]<T> as [<$name:camel Ty>]>::__DATA;
                }

                #[macro_export]
                macro_rules! [<$name:camel>] {
                    ($value:expr) => {
                        $crate::primitive::[<$name:camel Impl>]::<{ unsafe { core::mem::transmute::<$name, $base>(($value)) } }>
                    };
                }

                /// https://github.com/rust-lang/rust/pull/52234
                #[doc(hidden)] /** Not part of the public API */
                #[allow(unused_imports)]
                pub(crate) use [<$name:camel>] as [<_ $name:camel>];
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
