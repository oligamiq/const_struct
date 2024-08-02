use std::marker::PhantomData;

use const_struct_derive::ConstStruct;
use pre::{ConstStructData, ConstStructTy, Non};
use setting::{tester, WindowSettingManualTy, WINDOW_SETTING_MANUAL};

mod pre;
mod setting;

fn main() {
    tester::<setting::WindowSettingManualTy>();
}

#[derive(ConstStruct)]
struct TestSetting {
    a: Option<u32>,
}

impl TestSetting {
    pub const fn default() -> Self {
        Self { a: None }
    }
}

pub struct TestSettingManual {
    test_data: Option<u32>,
    test_data2: Option<u32>,
    test_data3: Option<u32>,
    test_data4: Option<u32>,
}

impl TestSettingManual {
    pub const fn __get_data(
        &self,
    ) -> pre::ConstStruct<Option<u32>, Option<u32>, Option<u32>, Option<u32>, Non> {
        pre::ConstStruct {
            __a: self.test_data,
            __b: self.test_data2,
            __c: self.test_data3,
            __d: self.test_data4,
            __e: Non,
        }
    }
}

impl ConstStructTy for TestSettingManual {
    type __A = Option<u32>;
    type __B = Option<u32>;
    type __C = Option<u32>;
    type __D = Option<u32>;
    type __E = Non;
}

pub trait TestSettingManualTy:
    ConstStructData<Option<u32>, Option<u32>, Option<u32>, Option<u32>, Non>
{
    const TEST_DATA: Option<u32> = Self::__A;
    const TEST_DATA2: Option<u32> = Self::__B;
    const TEST_DATA3: Option<u32> = Self::__C;
    const TEST_DATA4: Option<u32> = Self::__D;
    const TEST_DATA5: Non = Self::__E;
}

impl<T: ConstStructData<Option<u32>, Option<u32>, Option<u32>, Option<u32>, Non>>
    TestSettingManualTy for T
{
}
