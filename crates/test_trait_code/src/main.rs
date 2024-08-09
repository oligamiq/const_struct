use pre::{ConstStruct, ConstStructPrimImplType};
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

pub struct TestSettingManualImpl1<const U: u128, T> {
    __phantom: core::marker::PhantomData<T>,
}

impl ConstStructPrimImplType for TestSettingManual {
    type PrimType<const U: u128, TestSettingManualImpl2> = TestSettingManualImpl1<U, TestSettingManualImpl2>;
}

pub struct TestSettingManualImpl2<const U: u128, T> {
    __phantom: core::marker::PhantomData<T>,
}

impl<const S: u128> ConstStructPrimImplType for TestSettingManualImpl2<S, u32> {
    type PrimType<const U: u128, TestSettingManualImpl3> = TestSettingManualImpl2<U, TestSettingManualImpl3>;
}

pub struct TestSettingManualImpl3<const U: u128>;

impl<const S: u128> ConstStructPrimImplType for TestSettingManualImpl3<S> {
    type PrimType<const U: u128, T> = TestSettingManualImpl3<U>;
}

macro_rules! TestSettingManualImplType {
    ($value:expr) => {
        <TestSettingManual as ConstStructPrimImplType>::PrimType<{
            $value * 10
        }, TestSettingManualImpl2<{
            $value / 2
        }, TestSettingManualImpl2<{
            $value / 2
        }, TestSettingManualImpl3<{
            $value / 3
        }>>>>
    };
}

fn tester_prim() {
    let data: TestSettingManualImplType!(5);
    println!("{:?}", data);
}
