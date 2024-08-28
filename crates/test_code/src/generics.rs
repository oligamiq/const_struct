use const_struct::{const_struct, ConstStruct};

pub trait Float {}

impl Float for f32 {}

#[allow(dead_code)]
#[const_struct(macro_export)]
#[const_struct(Float: ::test_code::generics::Float)]
#[const_struct(Debug: ::std::fmt::Debug)]
#[derive(ConstStruct, Debug)]
pub struct TestGenerics<const T: usize, S: Float> {
    s: S,
}

#[const_struct]
const B: TestGenerics<7, f32> = TestGenerics { s: 0.0 };

#[cfg(test)]
pub mod test {
    use super::{BTy, Float, TestGenerics, TestGenericsTy};
    use core::fmt::Debug;

    pub fn tester_test_generics<
        const T: usize,
        S: Float + Copy + Debug,
        U: TestGenericsTy<T, S>,
    >() {
        no_std_compat::println!("tester_test_generics: {:?}", U::__DATA);
    }

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
    use crate::generics::{Float, TestGenerics, TestGenericsTy};
    use core::fmt::Debug;

    pub fn tester_test_generics<
        const T: usize,
        S: Float + Copy + Debug,
        U: TestGenericsTy<T, S>,
    >() {
        no_std_compat::println!("tester_test_generics: {:?}", U::__DATA);
    }

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

#[cfg(test)]
pub mod test3 {
    use const_struct::{call_with_generics, const_struct, primitive::TupleTy, ConstStruct};

    #[derive(ConstStruct)]
    pub struct TestSetting<const N: usize>;

    impl<const N: usize> core::fmt::Debug for TestSetting<N> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("TestSetting")
                .field("N", &N)
                .finish()
        }
    }

    pub fn tester<const N: usize, const N2: usize, A: TupleTy<(TestSetting<N>, TestSetting<N2>)>, const N3: usize, B: TupleTy<(f32, TestSetting<N3>)>>() {
        no_std_compat::println!("a: {:?}", A::__DATA);
        no_std_compat::println!("b: {:?}", B::__DATA);
    }

    #[const_struct]
    const B: TestSetting<0> = TestSetting;

    #[test]
    fn main() {
        call_with_generics!(tester::<(TestSetting!(BTy), TestSetting!(TestSetting::<2>)), (F32!(0.5), TestSetting!(8, TestSetting))>());
    }
}

// #[cfg(test)]
// pub mod test4 {
//     pub trait Float {}

//     impl Float for f32 {}

//     use const_struct::{const_struct, ConstStruct};

//     #[const_struct(Float: ::test_code::generics::test4::Float)]
//     #[derive(ConstStruct, Debug)]
//     pub struct TestGenerics<const T: usize, S: Float> {
//         s: S,
//     }

//     pub fn tester_test_generics<const T: usize, S: Float + core::fmt::Debug + Copy, U: TestGenericsTy<T, S>>() {
//         no_std_compat::println!("tester_test_generics: {:?}", U::__DATA);
//     }

//     // This is expected build failure
//     #[test]
//     fn test() {
//         tester_test_generics::<7, f32, TestGenerics!(TestGenerics { s: 0.0 })>();
//     }
// }
