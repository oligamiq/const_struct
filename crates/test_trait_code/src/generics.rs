use crate::hash_bridge::{HashBridge, HashBridgeBridge};
use crate::keeptype::KeepType;
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

impl<const T: usize, S: Float> KeepType<0> for TestStructWithFloatGenerics<T, S> {
    type Type = usize;
}

#[macro_export]
macro_rules! TestStructWithFloatGenerics {
    (@TestStructWithFloatGenericsGetGenericsData, @AdditionData($($addition_data:path: $addition_data2:path), *), $macro_path: path, $($arg:tt)*) => {
        {
            $macro_path!(
                @AdditionData($($addition_data: $addition_data2), *),
                @TestStructWithFloatGenericsGetGenericsData(
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

    ($s:path, $value:expr) => {
        HashBridge<{
            const NAME_HASH: u64 = str_hash(stringify!($value));

            type T = TestStructWithFloatGenerics<{
                const fn get_generic<const T: usize, S: Float + Copy>(_: TestStructWithFloatGenerics<{ T }, S>) -> usize {
                    T
                }

                get_generic($value)
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
            const fn get_generic<const T: usize, S: Float + Copy>(_: TestStructWithFloatGenerics<{ T }, S>) -> usize { T }
            get_generic($value)
        }, $s>
        >
    };

    ($value:expr) => {
        match_end_with!($value, Marker<{
            compile_error!("Expected a ???Ty")
        }>)
    };
}

#[cfg(test)]
mod tests {
    use crate::keeptype::Marker;
    use const_struct_derive::{call_with_generics, match_end_with};

    use crate::keeptype::{KeepType, KeepTypeConst};

    use super::*;

    fn caller<const T: usize, F: Float + Copy, U: TestStructWithFloatGenericsTy<T, F>>(
    ) -> TestStructWithFloatGenerics<T, F> {
        U::__DATA
    }

    const B: TestStructWithFloatGenerics<8, f32> = TestStructWithFloatGenerics {
        test_data: Some(1),
        test_data2: Some(Some(2)),
        test_data3: 3,
        test_data4: [0; 8],
        str: "test",
        float: 0.0,
    };

    pub struct BTy;

    impl PrimitiveTraits for BTy {
        type DATATYPE = TestStructWithFloatGenerics<8, f32>;
        const __DATA: <Self as PrimitiveTraits>::DATATYPE = B;
    }
    #[automatically_derived]
    #[doc(hidden)]
    impl KeepTypeConst<0> for BTy {
        type DATATYPE = <TestStructWithFloatGenerics<8, f32> as KeepType<0>>::Type;
        const N: Self::DATATYPE = { 8 };
    }
    #[automatically_derived]
    #[doc(hidden)]
    impl KeepType<1> for BTy {
        type Type = f32;
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

        let d = caller::<
            8,
            f32,
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
        >();

        let e = caller::<8, f32, TestStructWithFloatGenerics!(BTy)>();
    }
}
