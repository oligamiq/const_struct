use const_struct::{const_struct, ConstStruct};
use core::fmt::Debug;

#[allow(dead_code)]
pub trait Float {}

impl Float for f32 {}

#[allow(dead_code)]
#[const_struct(macro_export)]
#[derive(ConstStruct, Debug)]
pub struct TestNonGenerics {
    s: f32,
}

#[const_struct]
const B: TestNonGenerics = TestNonGenerics { s: 0.0 };

#[cfg(test)]
pub mod test {
    use super::{TestNonGenerics, TestNonGenericsTy, macros::TestNonGenerics};
    use const_struct::call_with_generics;

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
}
