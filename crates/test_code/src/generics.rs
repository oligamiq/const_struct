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

pub fn tester_test_generics<const T: usize, S: Float + Copy + Debug, U: TestGenericsTy<T, S>>() {
    no_std_compat::println!("tester_test_generics: {:?}", U::__DATA);
}

#[cfg(test)]
pub mod test {
    use super::{tester_test_generics, BTy, TestGenerics};

    #[test]
    fn test_generics() {
        tester_test_generics::<7, f32, TestGenerics!(7, f32, TestGenerics { s: 0.0 })>();
        const_struct::call_with_generics!(tester_test_generics::<
            TestGenerics!(7, f32, TestGenerics { s: 0.0 }),
        >());
        const_struct::call_with_generics!(tester_test_generics::<
            TestGenerics!(_, f32, TestGenerics::<7, f32> { s: 0.0 }),
        >());
        const_struct::call_with_generics!(tester_test_generics::<
            TestGenerics!(f32, TestGenerics::<7, f32> { s: 0.0 }),
        >());

        const_struct::call_with_generics!(tester_test_generics::<TestGenerics!(BTy)>());
        tester_test_generics::<7, f32, BTy>();
    }
}

#[cfg(test)]
pub mod test2 {
    use super::tester_test_generics;
    use crate::generics::TestGenerics;

    #[test]
    fn test_generics() {
        tester_test_generics::<7, f32, super::TestGenerics!(7, f32, TestGenerics { s: 0.0 })>();
        const_struct::call_with_generics!(tester_test_generics::<
            super::TestGenerics!(7, f32, TestGenerics { s: 0.0 }),
        >());
        const_struct::call_with_generics!(tester_test_generics::<
            super::TestGenerics!(_, f32, TestGenerics::<7, f32> { s: 0.0 }),
        >());
        const_struct::call_with_generics!(tester_test_generics::<
            super::TestGenerics!(f32, TestGenerics::<7, f32> { s: 0.0 }),
        >());

        const_struct::call_with_generics!(
            tester_test_generics::<super::TestGenerics!(super::BTy)>()
        );
        tester_test_generics::<7, f32, super::BTy>();
    }
}
