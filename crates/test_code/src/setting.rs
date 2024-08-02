use const_struct_derive::const_struct;

use crate::TestSetting;

#[const_struct]
const WINDOW_SETTING: TestSetting = {
    let mut c = TestSetting::default();
    c.a = Some(5);
    c
};
