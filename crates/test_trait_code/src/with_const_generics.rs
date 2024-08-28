use crate::match_underscore;
use crate::pre::{str_hash, PrimitiveTraits};

#[derive(Debug)]
pub struct TestStructWithGenerics<const T: usize> {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: [u8; T],
    str: &'static str,
}

pub trait TestStructWithGenericsTy<const T: usize>:
    PrimitiveTraits<DATATYPE = TestStructWithGenerics<{ T }>>
{
    const TEST_DATA: Option<u32> = <Self as PrimitiveTraits>::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = <Self as PrimitiveTraits>::__DATA.test_data2;
    const TEST_DATA3: u32 = <Self as PrimitiveTraits>::__DATA.test_data3;
    const TEST_DATA4: [u8; T] = <Self as PrimitiveTraits>::__DATA.test_data4;
    const STR: &'static str = <Self as PrimitiveTraits>::__DATA.str;
}

impl<U: PrimitiveTraits<DATATYPE = TestStructWithGenerics<{ T }>>, const T: usize>
    TestStructWithGenericsTy<{ T }> for U
{
}

#[macro_export]
macro_rules! TestStructWithGenerics {
    (@TestStructWithGenericsGetGenericsData, @AdditionData($($addition_data:path: $addition_data2:path), *), $macro_path: path, $($arg:tt)*) => {
        {
            $macro_path!(
                @AdditionData($($addition_data: $addition_data2), *),
                @TestStructWithGenericsGetGenericsData(
                    @AdditionData(
                        // ::const_struct::primitive::ConstStructPrimEnd: ConstStructPrimEnd,
                        // ::const_struct::primitive::ConstStructPrimQueue: ConstStructPrimQueue,
                        ::const_struct::keeptype::KeepType: KeepType,
                        ::const_struct::keeptype::KeepTypeConst: KeepTypeConst,
                        ::const_struct::primitive::PrimitiveTraits: PrimitiveTraits,
                        ::const_struct::primitive::str_hash: str_hash,
                        ::const_struct::primitive::HashBridge: HashBridge,
                        ::const_struct::primitive::HashBridgeBridge: HashBridgeBridge,
                        F32: F32
                    ),
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

            type T = TestStructWithGenerics<{
                match_underscore!($t, {
                    const fn get_generic<const T: usize>(_: TestStructWithGenerics<{ T }>) -> usize {
                        T
                    }

                    get_generic($value)
                })
            }>;

            impl HashBridgeBridge<NAME_HASH, {str_hash(file!())}, {column!()}, {line!()}> for T {
                type DATATYPE = T;
                const DATA: Self::DATATYPE = $value;
            }

            NAME_HASH
        }, {
            str_hash(file!())
        }, {
            column!()
        }, {
            line!()
        },
        TestStructWithGenerics<{
            match_underscore!($t, {
                const fn get_generic<const T: usize>(_: TestStructWithGenerics<{ T }>) -> usize {
                    T
                }

                get_generic($value)
            })
        }>
        >
    };
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use const_struct_derive::call_with_generics;

    use super::*;

    use crate::hash_bridge::{HashBridge, HashBridgeBridge};

    fn caller<const T: usize, U: TestStructWithGenericsTy<T> + Debug>() -> TestStructWithGenerics<T>
    {
        U::__DATA
    }

    #[test]
    fn test() {
        type T = TestStructWithGenerics!(
            _,
            TestStructWithGenerics {
                test_data: Some(1),
                test_data2: Some(Some(2)),
                test_data3: 3,
                test_data4: [0; 8],
                str: "test",
            }
        );

        let t: T = unsafe { core::mem::zeroed() };
        dbg!(t);

        assert_eq!(T::TEST_DATA, Some(1));
        assert_eq!(T::TEST_DATA2, Some(Some(2)));
        assert_eq!(T::TEST_DATA3, 3);
        assert_eq!(T::TEST_DATA4, [0; 8]);
        assert_eq!(T::STR, "test");

        caller::<
            8,
            TestStructWithGenerics!(
                _,
                TestStructWithGenerics {
                    test_data: Some(1),
                    test_data2: Some(Some(2)),
                    test_data3: 3,
                    test_data4: [0; 8],
                    str: "test",
                }
            ),
        >();

        let c = call_with_generics!(
            @AdditionData(
                ::const_struct::call_with_generics: call_with_generics
            ),
            caller::<
            TestStructWithGenerics!(TestStructWithGenerics {
                test_data: Some(1),
                test_data2: Some(Some(2)),
                test_data3: 3,
                test_data4: [0; 8],
                str: "test",
            }),
        >());
    }
}
