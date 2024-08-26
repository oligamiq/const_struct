use const_struct::call_with_generics;
use const_struct::{const_struct, ConstStruct};
use core::fmt::Debug;

pub trait Float {}

impl Float for f32 {}

#[const_struct(macro_export)]
#[derive(ConstStruct, Debug)]
pub struct TestNonGenerics {
    s: f32,
}

#[const_struct]
const B: TestNonGenerics = TestNonGenerics { s: 0.0 };

pub fn tester_test_generics<U: TestNonGenericsTy>() {
    no_std_compat::println!("tester_test_generics: {:?}", U::__DATA);
}

#[test]
fn test_generics() {
    tester_test_generics::<TestNonGenerics!(TestNonGenerics { s: 0.0 })>();
    call_with_generics!(tester_test_generics::<
        TestNonGenerics!(TestNonGenerics { s: 0.0 }),
    >());
}
