use pre::{ConstStruct, ConstStructPrimAny, ConstStructPrimBoxMem32, ConstStructPrimEnd};
use setting::WINDOW_SETTING_MANUAL;
use tester::{tester, tester_2};

mod pre;
mod primitive;
mod setting;
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
    test_data2: Option<u32>,
    test_data3: Option<u32>,
    test_data4: Option<u32>,
    str: &'static str,
}

pub trait TestSettingManualTy: ConstStruct<TestSettingManual> {
    const TEST_DATA: Option<u32> = Self::__DATA.test_data;
    const TEST_DATA2: Option<u32> = Self::__DATA.test_data2;
    const TEST_DATA3: Option<u32> = Self::__DATA.test_data3;
    const TEST_DATA4: Option<u32> = Self::__DATA.test_data4;
    const STR: &'static str = Self::__DATA.str;
}

impl<T: ConstStruct<TestSettingManual>> TestSettingManualTy for T {}

impl<const A: u32, const B: u32, const C: u32, const D: u32> ConstStruct<TestSettingManual>
    for ConstStructPrimAny<
        TestSettingManual,
        ConstStructPrimBoxMem32<
            A,
            ConstStructPrimBoxMem32<B, ConstStructPrimBoxMem32<C, ConstStructPrimEnd<D>>>,
        >,
    >
{
    const __DATA: TestSettingManual = TestSettingManual {
        test_data: Some(A as u32),
        test_data2: Some(B as u32),
        test_data3: Some(C as u32),
        test_data4: Some(D as u32),
        str: "abc_def",
    };
}

macro_rules! TestSettingManual {
    ($value:expr) => {
        ConstStructPrimAny<TestSettingManual,
        ConstStructPrimBoxMem32<{
            let value: TestSettingManual = $value;

            match value.test_data {
                Some(data) => data,
                None => 0,
            }
        }, ConstStructPrimBoxMem32<{
            let value: TestSettingManual = $value;

            match value.test_data2 {
                Some(data) => data,
                None => 0,
            }
        }, ConstStructPrimBoxMem32<{
            let value: TestSettingManual = $value;

            match value.test_data3 {
                Some(data) => data,
                None => 0,
            }
        }, ConstStructPrimEnd<{
            let value: TestSettingManual = $value;

            match value.test_data4 {
                Some(data) => data,
                None => 0,
            }
        }>>>>>
    };
}

impl TestSettingManual {
    pub const fn default() -> Self {
        Self {
            test_data: None,
            test_data2: None,
            test_data3: None,
            test_data4: None,
            str: "abc_def",
        }
    }
}

#[test]
fn tester_prim() {
    tester::<
        TestSettingManual!({
            TestSettingManual {
                test_data: Some(5),
                test_data2: Some(10),
                test_data3: None,
                test_data4: Some(15),
                str: "abc_def",
            }
        }),
    >();

    let ty: TestSettingManual!({
        TestSettingManual {
            test_data: Some(5),
            test_data2: Some(10),
            test_data3: None,
            test_data4: Some(15),
            str: "abc_def",
        }
    }) = ConstStructPrimAny {
        __phantom: core::marker::PhantomData,
    };
    println!("size: {:?}", core::mem::size_of_val(&ty));

}
