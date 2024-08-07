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

pub trait U32Ty {
    const __DATA: u32;
}

impl<T: U32TyImpl> U32Ty for u32 {
    const __DATA: u32 = T::__DATA;
}

pub trait U32TyImpl {
    const __DATA: Self;
}
