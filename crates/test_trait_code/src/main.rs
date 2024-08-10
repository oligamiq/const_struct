use core::str;
use std::ptr::slice_from_raw_parts;

use pre::ConstStructTraits;
use primitive::some::{OptionTy, PrimitiveTraits};
use setting::WINDOW_SETTING_MANUAL;
use struct_prim::{
    reduce_from_utf8, ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd, ConstStructPrimOption, ConstStructPrimU32, ConstStructPrimU8VecRef
};
use tester::{tester, tester_2};

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
pub struct TestSettingManual {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: Option<u32>,
    str: &'static str,
}

pub trait TestSettingManualTy: ConstStructTraits<TestSettingManual> {
    const TEST_DATA: Option<u32> = Self::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = Self::__DATA.test_data2;
    const TEST_DATA3: u32 = Self::__DATA.test_data3;
    const TEST_DATA4: Option<u32> = Self::__DATA.test_data4;
    const STR: &'static str = Self::__DATA.str;
}

impl<T: ConstStructTraits<TestSettingManual>> TestSettingManualTy for T {}

type TestSettingManualTyPrimWrapper<
    const A: bool,
    const B: u32,
    const C: bool,
    const D: bool,
    const E: u32,
    const F: u32,
    const G: bool,
    const H: u32,
    S,
> = ConstStructPrimAny<
    TestSettingManual,
    ConstStructPrimAny<
        ConstStructPrimOption<A, ConstStructPrimU32<B>>,
        ConstStructPrimAny<
            ConstStructPrimOption<C, ConstStructPrimOption<D, ConstStructPrimU32<E>>>,
            ConstStructPrimAny<
                ConstStructPrimU32<F>,
                ConstStructPrimAny<
                    ConstStructPrimOption<G, ConstStructPrimU32<H>>,
                    ConstStructPrimAny<S, ConstStructPrimEnd>,
                >,
            >,
        >,
    >,
>;

impl<
        const A: bool,
        const B: u32,
        const C: bool,
        const D: bool,
        const E: u32,
        const F: u32,
        const G: bool,
        const H: u32,
        S: ConstStructPrimData<Data = &'static [u8]>,
    > PrimitiveTraits for TestSettingManualTyPrimWrapper<A, B, C, D, E, F, G, H, S>
{
    type DATATYPE = TestSettingManual;
    const __DATA: Self::DATATYPE =
        <TestSettingManualTyPrimWrapper<A, B, C, D, E, F, G, H, S> as ConstStructTraits<
            TestSettingManual,
        >>::__DATA;
}

impl<
        const A: bool,
        const B: u32,
        const C: bool,
        const D: bool,
        const E: u32,
        const F: u32,
        const G: bool,
        const H: u32,
        S: ConstStructPrimData<Data = &'static [u8]>,
    > ConstStructTraits<TestSettingManual>
    for TestSettingManualTyPrimWrapper<A, B, C, D, E, F, G, H, S>
{
    const __DATA: TestSettingManual = {
        TestSettingManual {
            test_data: <ConstStructPrimOption::<A, ConstStructPrimU32<B>> as ConstStructPrimData>::__DATA,
            test_data2: <ConstStructPrimOption::<C, ConstStructPrimOption<D, ConstStructPrimU32<E>>> as ConstStructPrimData>::__DATA,
            test_data3: <ConstStructPrimU32::<F> as ConstStructPrimData>::__DATA,
            test_data4: <ConstStructPrimOption::<G, ConstStructPrimU32<H>> as ConstStructPrimData>::__DATA,
            str: reduce_from_utf8(S::__DATA),
        }
    };
}

macro_rules! TestSettingManual {
    ($value:expr) => {
        ConstStructPrimAny<TestSettingManual,
            ConstStructPrimAny<ConstStructPrimOption<{
                let v: TestSettingManual = $value;
                v.test_data.is_some()
            }, ConstStructPrimU32<{
                let v: TestSettingManual = $value;
                match v.test_data {
                    Some(data) => data,
                    None => 0,
                }
            }>>,
                ConstStructPrimAny<
                ConstStructPrimOption<
                    {
                        let v: TestSettingManual = $value;
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
                        let v: TestSettingManual = $value;
                        v.test_data3
                    }>,
                        ConstStructPrimAny<ConstStructPrimOption<{
                            let v: TestSettingManual = $value;
                            v.test_data4.is_some()
                        }, ConstStructPrimU32<{
                            let v: TestSettingManual = $value;
                            match v.test_data4 {
                                Some(data) => data,
                                None => 0,
                            }
                        }>>, ConstStructPrimAny<crate::struct_prim::StrWrapper2<{
                            let v: TestSettingManual = $value;
                            crate::struct_prim::str_to_u128::<0>(v.str)
                        }, {
                            let v: TestSettingManual = $value;
                            crate::struct_prim::str_to_u128::<16>(v.str)
                        }, {
                            let v: TestSettingManual = $value;
                            crate::struct_prim::str_to_u128::<32>(v.str)
                        }, {
                            let v: TestSettingManual = $value;
                            crate::struct_prim::str_to_u128::<48>(v.str)
                        }, {
                            let v: TestSettingManual = $value;
                            crate::struct_prim::str_to_u128::<64>(v.str)
                        }, {
                            let v: TestSettingManual = $value;
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

impl TestSettingManual {
    pub const fn default() -> Self {
        Self {
            test_data: None,
            test_data2: None,
            test_data3: 0,
            test_data4: None,
            str: "abc_def",
        }
    }
}

fn tester_with_option<T: OptionTy<Option<TestSettingManual>>>() {
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
        Some!(Some!(TestSettingManual!({
            TestSettingManual {
                test_data: Some(5),
                test_data2: Some(Some(10)),
                test_data3: 0,
                test_data4: Some(15),
                str: "",
                // str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
            }
        }))),
    >();

    let ty: TestSettingManual!({
        TestSettingManual {
            test_data: Some(5),
            test_data2: Some(None),
            test_data3: 0,
            test_data4: Some(15),
            str: "abc_def",
        }
    }) = ConstStructPrimAny {
        __phantom: core::marker::PhantomData,
    };

    println!("size: {:?}", core::mem::size_of_val(&ty));
}
