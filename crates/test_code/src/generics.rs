use const_struct::{const_struct, ConstStruct};

pub trait Float {}

impl Float for f32 {}

#[const_struct(macro_export)]
#[const_struct(Float: $crate::Float)]
#[const_struct(Debug: ::std::fmt::Debug)]
#[derive(ConstStruct)]
pub struct TestGenerics<S: Float> {
    s: S,
}

pub struct Y<const T: usize>
    where
{
    t: f32,
}

// #[const_struct]
// const B: TestGenerics<f32> = TestGenerics { s: 0.0 };

// pub fn tester_test_generics<S: Float>() -> S {
// }
