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
