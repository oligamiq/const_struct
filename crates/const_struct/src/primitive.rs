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

pub trait OptionTy<T> {
    const __DATA: Option<T>;
    const VALUE: Option<T> = Self::__DATA;
}

pub struct OptionImpl<T: PrimitiveTraits> {
    __phantom: core::marker::PhantomData<T>,
}

impl<T: PrimitiveTraits> OptionTy<T::DATATYPE> for OptionImpl<T> {
    const __DATA: Option<T::DATATYPE> = Some(<T as PrimitiveTraits>::__DATA);
}

impl<T: PrimitiveTraits> PrimitiveTraits for OptionImpl<T> {
    type DATATYPE = Option<T::DATATYPE>;
    const __DATA: Self::DATATYPE = Some(<T as PrimitiveTraits>::__DATA);
}

pub struct NoneImpl;

impl<T> OptionTy<T> for NoneImpl {
    const __DATA: Option<T> = None;
}

#[macro_export]
macro_rules! Some {
    ($value:ty) => {
        $crate::primitive::OptionImpl<$value>
    };
}

#[macro_export]
macro_rules! None {
    () => {
        $crate::primitive::NoneImpl
    };
}

pub const fn tester_inner<T: F32Ty>() -> f32 {
    T::VALUE
}

pub const fn tester_inner_u32<T: U32Ty>() -> u32 {
    T::VALUE
}

pub const fn tester_inner_option<T: OptionTy<f32>>() -> Option<f32> {
    T::VALUE
}

#[test]
pub fn call_tester() {
    let s = F32!(-0.5);
    debug_assert_eq!(core::mem::size_of_val(&s), 0);
    debug_assert_eq!(tester_inner::<F32!(-0.5)>(), -0.5);
    debug_assert_eq!(
        tester_inner_option::<Some!(F32!(-25.333))>(),
        Some(-25.333f32)
    );
    debug_assert_eq!(tester_inner_option::<None!()>(), None);
}
