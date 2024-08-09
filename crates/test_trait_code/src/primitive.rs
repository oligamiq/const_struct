#![allow(unused)]

use std::mem::transmute;

pub trait F32Ty {
    const __DATA: f32;
    const VALUE: f32 = Self::__DATA;
}

pub struct F32<const T: u32>;

impl<const T: u32> F32Ty for F32<T> {
    const __DATA: f32 = unsafe { transmute(T) };
}

macro_rules! F32 {
    ($value:expr) => {
        F32::<{ unsafe { transmute(($value) as f32) } }>
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
        const __DATA: Option<T::DATATYPE> = Some(T::__DATA);
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
