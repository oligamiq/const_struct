use crate::hash_bridge::{HashBridge, HashBridgeBridge};
use crate::match_underscore;
use crate::pre::{str_hash, PrimitiveTraits};

pub trait Float {}

impl Float for f32 {}

#[derive(Debug)]
pub struct TestStructWithFloatGenerics<const T: usize, S: Float> {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: [u8; T],
    str: &'static str,
    float: S,
}

pub trait TestStructWithFloatGenericsTy<const T: usize, S: Float + Copy>:
    PrimitiveTraits<DATATYPE = TestStructWithFloatGenerics<T, S>>
{
    const TEST_DATA: Option<u32> = <Self as PrimitiveTraits>::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = <Self as PrimitiveTraits>::__DATA.test_data2;
    const TEST_DATA3: u32 = <Self as PrimitiveTraits>::__DATA.test_data3;
    const TEST_DATA4: [u8; T] = <Self as PrimitiveTraits>::__DATA.test_data4;
    const STR: &'static str = <Self as PrimitiveTraits>::__DATA.str;
    const FLOAT: S = <Self as PrimitiveTraits>::__DATA.float;
}

impl<
        U: PrimitiveTraits<DATATYPE = TestStructWithFloatGenerics<T, S>>,
        const T: usize,
        S: Float + Copy,
    > TestStructWithFloatGenericsTy<T, S> for U
{
}

#[macro_export]
macro_rules! TestStructWithFloatGenerics {
    (@TestStructWithFloatGenericsGetGenericsData, $macro_path: path, $($arg:tt)*) => {
        {
            $macro_path!(
                @AdditionData(
                    // ::const_struct::primitive::ConstStructPrimEnd: ConstStructPrimEnd,
                    // ::const_struct::primitive::ConstStructPrimQueue: ConstStructPrimQueue,
                    ::const_struct::keeptype::KeepType: KeepType,
                    ::const_struct::keeptype::KeepTypeConst: KeepTypeConst,
                    ::const_struct::primitive::PrimitiveTraits: PrimitiveTraits,
                    ::const_struct::str_hash: str_hash,
                    ::const_struct::primitive::HashBridge: HashBridge,
                    ::const_struct::primitive::HashBridgeBridge: HashBridgeBridge,
                    F32: F32
                ),
                @TestStructWithFloatGenericsGetGenericsData(
                    struct,
                    const fn get_const_generics<const T: usize, S: Float + Copy>(_: TestStructWithFloatGenerics<{ T }, S>) {}
                ),
                $($arg)*
            )
        }
    };

    ($t:tt, $s:path, $value:expr) => {
        HashBridge<{
            const NAME_HASH: u64 = str_hash(stringify!($value));

            type T = TestStructWithFloatGenerics<{
                match_underscore!($t, {
                    const fn get_generic<const T: usize, S: Float + Copy>(_: TestStructWithFloatGenerics<{ T }, S>) -> usize {
                        T
                    }

                    get_generic($value)
                })
            }, $s>;

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
        TestStructWithFloatGenerics<{
            match_underscore!($t, {
                const fn get_generic<const T: usize, S: Float + Copy>(_: TestStructWithFloatGenerics<{ T }, S>) -> usize { T }
                get_generic($value)
            })
        }, $s>
        >
    };
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use const_struct_derive::call_with_generics;

    use super::*;

    fn caller<const T: usize, F: Float + Copy, U: TestStructWithFloatGenericsTy<T, F> + Debug>(
    ) -> TestStructWithFloatGenerics<T, F> {
        U::__DATA
    }

    #[test]
    fn test() {
        type T = TestStructWithFloatGenerics!(
            _,
            f32,
            TestStructWithFloatGenerics {
                test_data: Some(1),
                test_data2: Some(Some(2)),
                test_data3: 3,
                test_data4: [0; 8],
                str: "test",
                float: 0.0,
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
            f32,
            TestStructWithFloatGenerics!(
                _,
                f32,
                TestStructWithFloatGenerics {
                    test_data: Some(1),
                    test_data2: Some(Some(2)),
                    test_data3: 3,
                    test_data4: [0; 8],
                    str: "test",
                    float: 0.0,
                }
            ),
        >();

        let c = call_with_generics!(
            @AdditionData(
                ::const_struct::call_with_generics: call_with_generics
            ),
            caller::<
            TestStructWithFloatGenerics!(
                f32,
                TestStructWithFloatGenerics {
                    test_data: Some(1),
                    test_data2: Some(Some(2)),
                    test_data3: 3,
                    test_data4: [0; 8],
                    str: "test",
                    float: 0.0,
                }
            ),
        >());
    }
}
