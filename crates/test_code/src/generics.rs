use const_struct::{const_struct, ConstStruct};

pub trait Float {}

impl Float for f32 {}

#[const_struct(macro_export)]
#[derive(ConstStruct)]
pub struct TestGenerics {
    s: f32,
}

// #[const_struct]
// const B: TestGenerics<f32> = TestGenerics { s: 0.0 };

// pub fn tester_test_generics<S: Float>() -> S {
// }
