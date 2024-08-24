use super::PrimitiveTraits;
use core::mem::transmute;
use paste::paste;

macro_rules! PrimTraitBySizes {
    (@inner($dol:tt), $base:ident, $($name:ident),*) => {
        $(
            paste! {
                pub trait [<$name:camel Ty>] {
                    const __DATA: $name;
                    const VALUE: $name = <Self as [<$name:camel Ty>]>::__DATA;
                }

                #[derive(Debug, Copy, Clone)]
                pub struct [<$name:camel Impl>]<const T: $base>;

                impl<T: PrimitiveTraits<DATATYPE = $name>> [<$name:camel Ty>] for T {
                    const __DATA: $name = <T as PrimitiveTraits>::__DATA;
                }

                #[allow(clippy::useless_transmute)]
                #[allow(clippy::transmute_int_to_bool)]
                impl<const T: $base> PrimitiveTraits for [<$name:camel Impl>]<T> {
                    type DATATYPE = $name;
                    const __DATA: <Self as PrimitiveTraits>::DATATYPE = unsafe { transmute::<$base, $name>(T) };
                }

                #[macro_export]
                #[allow(clippy::useless_transmute)]
                // https://qiita.com/to-omer/items/a52bb4775ed04effde79
                macro_rules! [<$name:camel>] {
                    ([@ <$name:camel> GetGenericsData], $macro_path: path, $dol($dol args:tt)*) => {
                        $macro_path!([<$name:camel> GetGenericsData](), $dol($dol args)*)
                    };

                    ($value:expr) => {
                        $crate::primitive::[<$name:camel Impl>]::<{ unsafe { core::mem::transmute::<$name, $base>($value) } }>
                    };
                }

                /// https://github.com/rust-lang/rust/pull/52234
                #[doc(hidden)] /** Not part of the public API */
                #[allow(unused_imports)]
                pub(crate) use [<$name:camel>] as [<_ $name:camel>];
            }
        )*
    };
    ($size:literal, $($name:ident),*) => {
        $(
            paste! {
                PrimTraitBySizes!([<u $size>], $name);
            }
        )*
    };
    ($base:ident, $($name:ident),*) => {
        PrimTraitBySizes!(@inner($), $base, $($name),*);
    };
}

PrimTraitBySizes!(8, u8, i8, bool);
PrimTraitBySizes!(16, u16, i16);
PrimTraitBySizes!(32, u32, i32, f32, char);
PrimTraitBySizes!(64, u64, i64, f64);
PrimTraitBySizes!(128, u128, i128);
PrimTraitBySizes!(usize, usize, isize);

#[cfg(test)]
mod tests {
    use crate::primitive::{F32Ty, U32Ty, _F32 as F32, _U32 as U32};

    pub const fn tester_inner<T: F32Ty>() -> f32 {
        T::VALUE
    }

    pub const fn tester_inner_u32<T: U32Ty>() -> u32 {
        T::VALUE
    }

    #[test]
    pub fn call_tester() {
        let s = F32!(-0.5);
        assert_eq!(core::mem::size_of_val(&s), 0);
        assert_eq!(tester_inner::<F32!(-0.5)>(), -0.5);
        assert_eq!(tester_inner_u32::<U32!(0)>(), 0);
    }
}
