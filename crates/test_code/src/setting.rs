use const_struct::{const_compat, ConstStruct};
use no_std_compat::prelude::v1::*;

#[derive(ConstStruct, Debug, PartialEq)]
pub struct TestSetting {
    pub a: Option<u32>,
    abc_def: &'static str,
}

impl TestSetting {
    pub const fn default() -> Self {
        Self {
            a: None,
            abc_def: "abc_def",
        }
    }
}

#[const_compat(test_setting, #[cfg(not(feature = "dynamic"))])]
pub fn tester(test_setting: TestSetting) {
    let t = test_setting.abc_def;
    println!("{:?}", t);

    tester_inner(test_setting.a.unwrap());

    let test_setting = TestSetting::default();

    println!("{:?}", test_setting);
}
// integers, `bool` and `char`

// #[const_compat(i, #[cfg(not(feature = "dynamic"))])]
fn tester_inner(i: u32) {
    println!("{:?}", i);
}
