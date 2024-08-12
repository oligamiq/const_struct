use crate::{pre::ConstStructTraits, primitive::some::PrimitiveTraits, TestSettingManual};

pub const WINDOW_SETTING_MANUAL: TestSettingManual<20> = TestSettingManual {
    test_data: Some(5),
    test_data2: None,
    test_data3: 0,
    test_data4: [0; 20],
    str: "abc_def",
};

pub struct WindowSettingManualTy;

impl PrimitiveTraits for WindowSettingManualTy {
    type DATATYPE = TestSettingManual<20>;
    const __DATA: Self::DATATYPE = WINDOW_SETTING_MANUAL;
}
