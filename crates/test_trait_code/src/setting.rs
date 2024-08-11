use crate::{pre::ConstStructTraits, TestSettingManual};

pub const WINDOW_SETTING_MANUAL: TestSettingManual = TestSettingManual {
    test_data: Some(5),
    test_data2: None,
    test_data3: 0,
    test_data4: [0; 20],
    str: "abc_def",
};

pub struct WindowSettingManualTy;

impl ConstStructTraits<TestSettingManual> for WindowSettingManualTy {
    const __DATA: TestSettingManual = WINDOW_SETTING_MANUAL;
}
