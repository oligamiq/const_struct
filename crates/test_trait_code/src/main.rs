use pre::{ConstStruct, ConstStructPrimImplType, TestSettingManualEnd, TestSettingManualBox};
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

impl<const A: u32, const B: u32, const C: u32, const D: u32> ConstStruct<TestSettingManual> for TestSettingManualBox<A, TestSettingManualBox<B, TestSettingManualBox<C, TestSettingManualEnd<D>>>> {
    const __DATA: TestSettingManual = TestSettingManual {
        test_data: Some(A as u32),
        test_data2: Some(B as u32),
        test_data3: Some(C as u32),
        test_data4: None,
        str: "abc_def",
    };
}

macro_rules! TestSettingManualImplType {
    ($value:expr) => {
        TestSettingManualBox<{
            $value * 10
        }, TestSettingManualBox<{
            $value / 2
        }, TestSettingManualBox<{
            $value / 2
        }, TestSettingManualEnd<{
            $value / 3
        }>>>>
    };
}

#[test]
fn tester_prim() {
    tester::<TestSettingManualImplType!(5)>();
}
