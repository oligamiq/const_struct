use const_struct::{const_compat, ConstStruct};
use no_std_compat::prelude::v1::*;

#[derive(ConstStruct, Debug)]
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

// pub fn tester<T: TestSettingTy>() {
//     let t = T::__DATA.abc_def;
//     println!("{:?}", t);
// }

// #[cfg(feature = "test")]
// fn test1() {
//     let t = TestSetting::default();
//     println!("{:?}", t);
// }

#[const_compat(test_setting, #[cfg(not(feature = "dynamic"))])]
pub fn tester(test_setting: TestSetting) {
    let t = test_setting.abc_def;
    println!("{:?}", t);

    tester_inner(test_setting.a.unwrap());

    let test_setting = TestSetting::default();

    println!("{:?}", test_setting);
}

fn tester_inner(i: u32) {
    println!("{:?}", i);
}
