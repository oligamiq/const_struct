use core::str;

use primitive::{some::{OptionTy, PrimitiveTraits}, F32Ty};
use setting::WINDOW_SETTING_MANUAL;
use struct_prim::{
    ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd,
    ConstStructPrimOption, ConstStructPrimU32, ConstStructPrimU8Vec, ConstStructPrimU8VecLimit,
};

use tester::{tester, tester_2};

mod generics;
mod generics_with_const;
mod pre;
mod primitive;
mod setting;
mod struct_prim;
mod tester;

fn main() {
    tester::<setting::WindowSettingManualTy>();
    tester_2(WINDOW_SETTING_MANUAL);

    match {
        #[cfg(feature = "dynamic")]
        {
            Some("data")
        }
        #[cfg(not(feature = "dynamic"))]
        {
            Option::<&str>::None
        }
    } {
        Some(data) => println!("data: {}", data),
        None => println!("data: None"),
    }
}

#[derive(Debug)]
pub struct TestSettingManual<const T: usize> {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: [u8; T],
    str: &'static str,
}

impl<const T: usize> TestSettingManual<T> {
    // pub const fn get_const_generics_t() -> usize {
    //     T
    // }

    pub const fn get_const_generics_t<const T2: usize>(_: TestSettingManual<T2>) -> usize {
        T2
    }
}

pub trait TestSettingManualTy<const T: usize>: PrimitiveTraits<DATATYPE = TestSettingManual<T>> {
    const TEST_DATA: Option<u32> = <Self as PrimitiveTraits>::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = Self::__DATA.test_data2;
    const TEST_DATA3: u32 = Self::__DATA.test_data3;
    const TEST_DATA4: [u8; T] = Self::__DATA.test_data4;
    const STR: &'static str = Self::__DATA.str;
}

impl<const T: usize, U: PrimitiveTraits<DATATYPE = TestSettingManual<T>>> TestSettingManualTy<T> for U {}

type TestSettingManualTyPrimWrapper<const T: usize, A, B, C, D, S> = ConstStructPrimAny<
    TestSettingManual<T>,
    ConstStructPrimAny<
        A,
        ConstStructPrimAny<
            B,
            ConstStructPrimAny<C, ConstStructPrimAny<D, ConstStructPrimAny<S, ConstStructPrimEnd>>>,
        >,
    >,
>;

impl<
        const T: usize,
        A: ConstStructPrimData<Data = Option<u32>>,
        B: ConstStructPrimData<Data = Option<Option<u32>>>,
        C: ConstStructPrimData<Data = u32>,
        D: ConstStructPrimData<Data = [u8; T]>,
        S: ConstStructPrimData<Data = &'static str>,
    > PrimitiveTraits for TestSettingManualTyPrimWrapper<T, A, B, C, D, S>
{
    type DATATYPE = TestSettingManual<T>;
    const __DATA: Self::DATATYPE = {
        TestSettingManual {
            test_data: <A as ConstStructPrimData>::__DATA,
            test_data2: <B as ConstStructPrimData>::__DATA,
            test_data3: <C as ConstStructPrimData>::__DATA,
            test_data4: <D as ConstStructPrimData>::__DATA,
            str: <S as ConstStructPrimData>::__DATA,
        }
    };
}

macro_rules! TestSettingManual {
    ($value:expr) => {
        ConstStructPrimAny<TestSettingManual<{
            TestSettingManual::<0>::get_const_generics_t($value)
        }>,
            ConstStructPrimAny<ConstStructPrimOption<{
                let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                v.test_data.is_some()
            }, ConstStructPrimU32<{
                let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                match v.test_data {
                    Some(data) => data,
                    None => 0,
                }
            }>>,
                ConstStructPrimAny<
                ConstStructPrimOption<
                    {
                        let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                        v.test_data2.is_some()
                    },
                    ConstStructPrimOption<{
                        match $value.test_data2 {
                            Some(data) => data.is_some(),
                            None => false,
                        }
                    }, ConstStructPrimU32<{
                        match $value.test_data2 {
                            Some(data) => match data {
                                Some(data) => data,
                                None => 0,
                            },
                            None => 0,
                        }
                    }>>>,
                    ConstStructPrimAny<ConstStructPrimU32<{
                        let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;

                        v.test_data3
                    }>,
                        ConstStructPrimAny<
                        ConstStructPrimU8VecLimit<
                            {
                                let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                                v.test_data4.len()
                            },
                            ConstStructPrimU8Vec<
                            {
                                let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                                crate::struct_prim::vec_u8_to_u128::<16>(&v.test_data4)
                            }
                            ,32
                            , ConstStructPrimU8Vec<{
                                let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                                crate::struct_prim::vec_u8_to_u128::<0>(&v.test_data4)
                            }, 16, ConstStructPrimEnd>
                            >
                        >
                        , ConstStructPrimAny<crate::struct_prim::StrWrapper5<{
                            let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                            crate::struct_prim::str_to_u128::<0>(v.str)
                        }, {
                            let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                            crate::struct_prim::str_to_u128::<16>(v.str)
                        }, {
                            let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                            crate::struct_prim::str_to_u128::<32>(v.str)
                        }, {
                            let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                            crate::struct_prim::str_to_u128::<48>(v.str)
                        }, {
                            let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                            crate::struct_prim::str_to_u128::<64>(v.str)
                        }, {
                            let v: TestSettingManual<{TestSettingManual::<0>::get_const_generics_t($value)}> = $value;
                            v.str.len()
                        }
                        >, ConstStructPrimEnd>
                        >
                    >
                >
            >
        >
    };
}

impl TestSettingManual<20> {
    pub const fn default() -> Self {
        Self {
            test_data: None,
            test_data2: None,
            test_data3: 0,
            test_data4: [
                1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
            ],
            str: "abc_def",
        }
    }
}

fn tester_with_option<const U: usize, T: OptionTy<Option<TestSettingManual<U>>>>() {
    let t = T::__DATA;
    println!("{:?}", t);
    println!("{:?}", T::__DATA);
}

#[test]
fn call_tester_prim() {
    tester_prim();
}

pub fn tester_prim() {
    tester_with_option::<
        14,
        Some!(Some!(TestSettingManual!({
            TestSettingManual {
                test_data: Some(5),
                test_data2: Some(Some(10)),
                test_data3: 0,
                test_data4: [1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41],
                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
            }
        }))),
    >();

    let ty: TestSettingManual!({
        TestSettingManual {
            test_data: Some(5),
            test_data2: Some(None),
            test_data3: 0,
            test_data4: [
                1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
            ],
            str: "abc_def",
        }
    }) = ConstStructPrimAny {
        __phantom: core::marker::PhantomData,
    };

    println!("size: {:?}", core::mem::size_of_val(&ty));
}

const PI: f32 = 3.14159;

struct PiTy;

impl PrimitiveTraits for PiTy {
    type DATATYPE = f32;
    const __DATA: <Self as PrimitiveTraits>::DATATYPE = PI;
}

fn tester_pi<T: F32Ty>() {
    println!("PI: {}", T::__DATA);
}

#[test]
fn call_tester_pi() {
    tester_pi::<PiTy>();
}
