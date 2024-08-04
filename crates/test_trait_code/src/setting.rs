use crate::{pre::ConstStruct, TestSettingManual};

pub const WINDOW_SETTING_MANUAL: TestSettingManual = TestSettingManual {
    test_data: Some(5),
    test_data2: None,
    test_data3: None,
    test_data4: None,
    str: "abc_def",
};

pub struct WindowSettingManualTy;

impl ConstStruct<TestSettingManual> for WindowSettingManualTy {
    const __DATA: TestSettingManual = WINDOW_SETTING_MANUAL;
}
