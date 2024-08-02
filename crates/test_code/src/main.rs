use std::marker::PhantomData;

use const_struct_derive::ConstStruct;
use pre::{ConstStructImplData, ConstStructImplTy, DefaultNone};

mod pre;
mod setting;

fn main() {}

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

impl ConstStructImplTy for TestSettingManual {
    type __A = Option<u32>;
    type __B = Option<u32>;
    type __C = Option<u32>;
    type __D = Option<u32>;
    type __E = DefaultNone;
}

pub trait TestSettingManualTy:
    ConstStructImplData<
        <TestSettingManual as ConstStructImplTy>::__A,
        <TestSettingManual as ConstStructImplTy>::__B,
        <TestSettingManual as ConstStructImplTy>::__C,
        <TestSettingManual as ConstStructImplTy>::__D,
        <TestSettingManual as ConstStructImplTy>::__E,
    >
{
    const TEST_DATA: <TestSettingManual as ConstStructImplTy>::__A = ConstStructImplData::__A;
    const TEST_DATA2: <TestSettingManual as ConstStructImplTy>::__B = ConstStructImplData::__B;
    const TEST_DATA3: <TestSettingManual as ConstStructImplTy>::__C = ConstStructImplData::__C;
    const TEST_DATA4: <TestSettingManual as ConstStructImplTy>::__D = ConstStructImplData::__D;
    const TEST_DATA5: <TestSettingManual as ConstStructImplTy>::__E = ConstStructImplData::__E;
}
