use const_struct_derive::const_struct;

use crate::{
    pre::{ConstStructImplData, ConstStructImplTy},
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

impl ConstStructImplData for WindowSettingManualTy {
    const __A: <Self as ConstStructImplTy>::__A = WINDOW_SETTING_MANUAL.__get_data().__a;
    const __B: <Self as ConstStructImplTy>::__B = WINDOW_SETTING_MANUAL.__get_data().__b;
    const __C: <Self as ConstStructImplTy>::__C = WINDOW_SETTING_MANUAL.__get_data().__c;
    const __D: <Self as ConstStructImplTy>::__D = WINDOW_SETTING_MANUAL.__get_data().__d;
    const __E: <Self as ConstStructImplTy>::__E = WINDOW_SETTING_MANUAL.__get_data().__e;
}
