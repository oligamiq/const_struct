use const_struct::{const_struct, ConstStruct};
use core::fmt::Debug;

pub trait Float {}

impl Float for f32 {}

#[const_struct(macro_export)]
#[const_struct(Float: ::test_code::generics::Float)]
#[const_struct(Debug: ::std::fmt::Debug)]
#[derive(ConstStruct, Debug)]
pub struct TestGenerics<const T: usize, S: Float> {
    s: S,
}

#[const_struct]
const B: TestGenerics<7, f32> = TestGenerics { s: 0.0 };

pub fn tester_test_generics<S: Float + Copy + Debug, const T: usize, U: TestGenericsTy<T, S>>() {
    no_std_compat::println!("tester_test_generics: {:?}", U::__DATA);
}

// #[test]
// pub fn test_test_generics() {
//     // tester_test_generics::<f32, 7, BTy>();
//     tester_test_generics::<_, {
//         const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
//             A
//         }

//         get_const_generics_a(B)
//     }, BTy>();
// }
