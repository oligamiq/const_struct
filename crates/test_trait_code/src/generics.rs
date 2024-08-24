use crate::pre::{str_hash, PrimitiveTraits};
use crate::match_underscore;

#[derive(Debug)]
pub struct TestStructWithGenerics<const T: usize> {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: [u8; T],
    str: &'static str,
}

pub trait TestStructWithGenericsTy<const T: usize>:
    PrimitiveTraits<DATATYPE = TestStructWithGenerics<{T}>>
{
    const TEST_DATA: Option<u32> = <Self as PrimitiveTraits>::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = <Self as PrimitiveTraits>::__DATA.test_data2;
    const TEST_DATA3: u32 = <Self as PrimitiveTraits>::__DATA.test_data3;
    const TEST_DATA4: [u8; T] = <Self as PrimitiveTraits>::__DATA.test_data4;
    const STR: &'static str = <Self as PrimitiveTraits>::__DATA.str;
}

impl<U: PrimitiveTraits<DATATYPE = TestStructWithGenerics<{T}>>, const T: usize> TestStructWithGenericsTy<{T}> for U {}

#[macro_export]
macro_rules! TestStructWithGenerics {
    (@TestStructWithGenericsGetGenericsData, $macro_path: path, $($arg:tt)*) => {
        {
            $macro_path!(
                @AdditionData(
                    // ::const_struct::primitive::ConstStructPrimEnd: ConstStructPrimEnd,
                    // ::const_struct::primitive::ConstStructPrimQueue: ConstStructPrimQueue,
                    ::const_struct::keeptype::KeepType: KeepType,
                    ::const_struct::keeptype::KeepTypeConst: KeepTypeConst,
                    ::const_struct::primitive::PrimitiveTraits: PrimitiveTraits,
                    ::const_struct::str_hash: str_hash,
                    ::const_struct::HashBridge: HashBridge,
                    F32: F32
                ),
                @TestStructWithGenericsGetGenericsData(
                    struct,
                    const fn get_const_generics<const T: usize>(_: TestStructWithGenerics<{ T }>) {}
                ),
                $($arg)*
            )
        }
    };

    ($t:tt, $value:expr) => {
        HashBridge<{
            const NAME_HASH: u64 = str_hash(stringify!($value));

            impl PrimitiveTraits for HashBridge<NAME_HASH, {str_hash(file!())}, {column!()}, {line!()}> {
                type DATATYPE = TestStructWithGenerics<{
                    match_underscore!($t, {
                        const fn get_generic<const T: usize>(_: TestStructWithGenerics<{ T }>) -> usize {
                            T
                        }

                        get_generic($value)
                    })
                }>;
                const __DATA: Self::DATATYPE = $value;
            }

            NAME_HASH
        }, {
            str_hash(file!())
        }, {
            column!()
        }, {
            line!()
        }>
    };
}

#[cfg(test)]
mod tests {
    use std::{arch::x86_64, fmt::Debug};

    use const_struct_derive::call_with_generics;

    use super::*;
    use crate::pre::HashBridge;

    fn caller<const T: usize, U: TestStructWithGenericsTy<T> + Debug>() -> TestStructWithGenerics<T> {
        U::__DATA
    }

    #[test]
    fn test() {
        type T = TestStructWithGenerics!(_, TestStructWithGenerics {
            test_data: Some(1),
            test_data2: Some(Some(2)),
            test_data3: 3,
            test_data4: [0; 8],
            str: "test",
        });

        let t: T = unsafe { core::mem::zeroed() };
        dbg!(t);

        assert_eq!(T::TEST_DATA, Some(1));
        assert_eq!(T::TEST_DATA2, Some(Some(2)));
        assert_eq!(T::TEST_DATA3, 3);
        assert_eq!(T::TEST_DATA4, [0; 8]);
        assert_eq!(T::STR, "test");

        caller::<8, TestStructWithGenerics!(_, TestStructWithGenerics {
            test_data: Some(1),
            test_data2: Some(Some(2)),
            test_data3: 3,
            test_data4: [0; 8],
            str: "test",
        })>();

        let c = call_with_generics!(caller::<TestStructWithGenerics!(TestStructWithGenerics {
            test_data: Some(1),
            test_data2: Some(Some(2)),
            test_data3: 3,
            test_data4: [0; 8],
            str: "test",
        })>());
    }
}
