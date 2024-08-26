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

#[test]
fn test_generics() {
    // tester_test_generics::<7, f32, TestGenerics!(7, f32, TestGenerics { s: 0.0 })>();

    tester_test_generics::<
    7,
    f32,
    ::const_struct::primitive::HashBridge<
        {
            const NAME_HASH: u64 = ::const_struct::primitive::str_hash(
                "TestGenerics { s: 0.0 }",
            );
            impl ::const_struct::PrimitiveTraits
            for ::const_struct::primitive::HashBridge<
                NAME_HASH,
                {
                    ::const_struct::primitive::str_hash(
                        "crates\\test_code\\src\\generics.rs",
                    )
                },
                { 36u32 },
                { 25u32 },
            > {
                type DATATYPE = TestGenerics<{ 7 }, f32>;
                const __DATA: Self::DATATYPE = { TestGenerics { s: 0.0 } };
            }
            NAME_HASH
        },
        {
            ::const_struct::primitive::str_hash(
                "crates\\test_code\\src\\generics.rs",
            )
        },
        { 36u32 },
        { 25u32 },
        >,
    >();
}
