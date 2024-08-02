use const_struct_derive::const_struct;

use crate::{
    pre::{ConstStructData, ConstStructTy},
    TestSetting, TestSettingManual, TestSettingManualTy,
};

#[const_struct]
const WINDOW_SETTING: TestSetting = {
    let mut c = TestSetting::default();
    c.a = Some(5);
    c
};

pub fn tester<T: TestSettingManualTy>() {
    let t = T::TEST_DATA;
    println!("{:?}", t);
}

pub const WINDOW_SETTING_MANUAL: TestSettingManual = TestSettingManual {
    test_data: Some(5),
    test_data2: None,
    test_data3: None,
    test_data4: None,
};

pub struct WindowSettingManualTy;

impl
    ConstStructData<
        <TestSettingManual as ConstStructTy>::__A,
        <TestSettingManual as ConstStructTy>::__B,
        <TestSettingManual as ConstStructTy>::__C,
        <TestSettingManual as ConstStructTy>::__D,
        <TestSettingManual as ConstStructTy>::__E,
    > for WindowSettingManualTy
{
    const __A: <TestSettingManual as ConstStructTy>::__A =
        WINDOW_SETTING_MANUAL.__get_data().__a;
    const __B: <TestSettingManual as ConstStructTy>::__B =
        WINDOW_SETTING_MANUAL.__get_data().__b;
    const __C: <TestSettingManual as ConstStructTy>::__C =
        WINDOW_SETTING_MANUAL.__get_data().__c;
    const __D: <TestSettingManual as ConstStructTy>::__D =
        WINDOW_SETTING_MANUAL.__get_data().__d;
    const __E: <TestSettingManual as ConstStructTy>::__E =
        WINDOW_SETTING_MANUAL.__get_data().__e;
}
