use const_struct_derive::const_struct;

use crate::{TestSetting, TestSettingManual, TestSettingManualTyImpl, TestSettingManualTyImplData};

#[const_struct]
const WINDOW_SETTING: TestSetting = {
    let mut c = TestSetting::default();
    c.a = Some(5);
    c
};

fn tester() {
    let t: TestSetting = WINDOW_SETTING;
}

const WINDOW_SETTING_MANUAL: TestSettingManual = TestSettingManual {
    test_data: Some(5),
};

struct WindowSettingManualTy;

impl TestSettingManualTyImpl for WindowSettingManualTy {
    const __A: <TestSettingManual as TestSettingManualTyImplData>::__A = WINDOW_SETTING_MANUAL._;
}
