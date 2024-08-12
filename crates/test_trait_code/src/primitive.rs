#![allow(unused)]

use core::mem::transmute;

use some::PrimitiveTraits;

use crate::{pre::ConstStructTraits, struct_prim::ConstStructPrimData};

pub trait F32Ty {
    const __DATA: f32;
    const VALUE: f32 = Self::__DATA;
}

#[derive(Debug, Copy, Clone)]
pub struct F32Impl<const T: u32>;

impl<T: PrimitiveTraits<DATATYPE = f32>> F32Ty for T {
    const __DATA: f32 = <T as PrimitiveTraits>::__DATA;
}

impl<const T: u32> PrimitiveTraits for F32Impl<T> {
    type DATATYPE = f32;
    const __DATA: f32 = unsafe { transmute(T) };
}

impl<const T: u32> ConstStructPrimData for F32Impl<T> {
    type Data = f32;
    const __DATA: f32 = <F32Impl<T> as F32Ty>::__DATA;
}

#[macro_export]
macro_rules! F32 {
    ($value:expr) => {
        $crate::primitive::F32Impl::<{ unsafe { core::mem::transmute::<f32, u32>($value) } }>
    };
}

pub trait U32Ty {
    const __DATA: u32;
    const VALUE: u32 = Self::__DATA;
}

pub struct U32Impl<const T: u32>;
impl<const T: u32> U32Ty for U32Impl<T> {
    const __DATA: u32 = unsafe { transmute(T) };
}

impl<U: U32Ty, const T: u32> ConstStructTraits<U32Impl<T>> for U {
    const __DATA: U32Impl<T> = U32Impl::<T>;
}

impl<const T: u32> PrimitiveTraits for U32Impl<T> {
    type DATATYPE = u32;
    const __DATA: Self::DATATYPE = <U32Impl<T> as U32Ty>::__DATA;
}

#[macro_export]
macro_rules! U32 {
    ($value:expr) => {
        $crate::primitive::U32Impl::<{ unsafe { core::mem::transmute(($value) as u32) } }>
    };
}

pub mod some {
    pub trait PrimitiveTraits {
        type DATATYPE;
        const __DATA: Self::DATATYPE;
    }

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
        $crate::primitive::some::OptionImpl<$value>
    };
}

    #[macro_export]
    macro_rules! None {
        () => {
            $crate::primitive::NoneImpl
        };
    }
}

pub const fn tester_inner<T: F32Ty>() -> f32 {
    T::__DATA
}

#[test]
pub fn call_tester() {
    debug_assert_eq!(tester_inner::<F32!(-0.5)>(), -0.5);
    debug_assert_eq!(tester_inner::<F32!(-25.333)>(), -25.333);
}

// struct RestrictInner<const B: bool>;
// trait Restrict {}
// impl Restrict for RestrictInner<true> {}

// pub trait F32Ty {
//     const __DATA: f32;
//     const VALUE: f32 = Self::__DATA;
// }

// pub struct Mem32<const T: u32>;

// pub trait Mem32Ty<T> {
//     const __DATA: T;
//     const VALUE: T = Self::__DATA;
// }

// impl<U, const T: u32> Mem32Ty<U> for Mem32<T>
//     where RestrictInner<{ core::mem::size_of::<U>() == 32 }>: Restrict
// {
//     const __DATA: U = unsafe { transmute(T) };
// }
