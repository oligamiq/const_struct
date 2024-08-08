#![allow(unused)]

use std::mem::transmute;

pub trait F32Ty {
    const __DATA: f32;
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
