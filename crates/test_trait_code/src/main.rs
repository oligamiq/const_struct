use core::str;
use std::{ptr::slice_from_raw_parts, str};

use pre::ConstStructTraits;
use primitive::some::{OptionTy, PrimitiveTraits};
use setting::WINDOW_SETTING_MANUAL;
use struct_prim::{
    ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd, ConstStructPrimOption,
    ConstStructPrimRef, ConstStructPrimStr, ConstStructPrimU32,
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
    const I: usize,
    const J: usize,
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
                    ConstStructPrimAny<
                        ConstStructPrimRef<I, ConstStructPrimStr<J>>,
                        ConstStructPrimEnd,
                    >,
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
        const I: usize,
        const J: usize,
    > PrimitiveTraits for TestSettingManualTyPrimWrapper<A, B, C, D, E, F, G, H, I, J>
{
    type DATATYPE = TestSettingManual;
    const __DATA: Self::DATATYPE =
        <TestSettingManualTyPrimWrapper<A, B, C, D, E, F, G, H, I, J> as ConstStructTraits<
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
        const I: usize,
        const J: usize,
    > ConstStructTraits<TestSettingManual>
    for TestSettingManualTyPrimWrapper<A, B, C, D, E, F, G, H, I, J>
{
    const __DATA: TestSettingManual = {
        TestSettingManual {
            test_data: <ConstStructPrimOption::<A, ConstStructPrimU32<B>> as ConstStructPrimData>::__DATA,
            test_data2: <ConstStructPrimOption::<C, ConstStructPrimOption<D, ConstStructPrimU32<E>>> as ConstStructPrimData>::__DATA,
            test_data3: <ConstStructPrimU32::<F> as ConstStructPrimData>::__DATA,
            test_data4: <ConstStructPrimOption::<G, ConstStructPrimU32<H>> as ConstStructPrimData>::__DATA,
            str: unsafe { str::from_utf8_unchecked(&*slice_from_raw_parts(<ConstStructPrimRef::<I, ConstStructPrimStr<J>> as ConstStructPrimData>::__DATA as *const _, J)) },
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
                        }>>,
                            ConstStructPrimAny<ConstStructPrimRef<{
                                let v: TestSettingManual = $value;
                                unsafe { core::mem::transmute(str.as_ptr()) }
                            }, ConstStructPrimStr<{
                                let v: TestSettingManual = $value;
                                v.str.len()
                            }>>,
                                ConstStructPrimEnd
                            >
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
fn tester_prim() {
    tester_with_option::<
        Some!(Some!(TestSettingManual!({
            TestSettingManual {
                test_data: Some(5),
                test_data2: Some(Some(10)),
                test_data3: 0,
                test_data4: Some(15),
                str: "abc_def",
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
