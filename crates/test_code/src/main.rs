#![no_std]

use const_struct::const_struct;
use setting::{tester, TestSetting};

mod primitive;
mod setting;

mod generics;
mod non_generics;
mod parse_value;

#[const_struct]
const WINDOW_SETTING: TestSetting = {
    let mut c = TestSetting::default();
    c.a = Some(5);
    c
};

fn main() {
    #[cfg(feature = "dynamic")]
    tester::<WindowSettingTy>();

    #[cfg(not(feature = "dynamic"))]
    tester(WINDOW_SETTING);
}
