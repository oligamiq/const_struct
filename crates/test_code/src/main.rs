use std::marker::PhantomData;

use const_struct_derive::ConstStruct;
use pre::{ConstStructImplData, ConstStructImplTy, DefaultNone};
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
    ) -> pre::ConstStructImpl<Option<u32>, Option<u32>, Option<u32>, Option<u32>, DefaultNone> {
        pre::ConstStructImpl {
            __a: self.test_data,
            __b: self.test_data2,
            __c: self.test_data3,
            __d: self.test_data4,
            __e: DefaultNone,
        }
    }
}

impl<T: ConstStructImplData> ConstStructImplTy for T {
    type __A = Option<u32>;
    type __B = Option<u32>;
    type __C = Option<u32>;
    type __D = Option<u32>;
    type __E = DefaultNone;
}

pub trait TestSettingManualTy: ConstStructImplData {
    const TEST_DATA: <Self as ConstStructImplTy>::__A = Self::__A;
    const TEST_DATA2: <Self as ConstStructImplTy>::__B = Self::__B;
    const TEST_DATA3: <Self as ConstStructImplTy>::__C = Self::__C;
    const TEST_DATA4: <Self as ConstStructImplTy>::__D = Self::__D;
    const TEST_DATA5: <Self as ConstStructImplTy>::__E = Self::__E;
}

impl<T: ConstStructImplData> TestSettingManualTy for T {}
