use crate::{TestSettingManual, TestSettingManualTy};

pub fn tester<T: TestSettingManualTy>() {
    let t = T::TEST_DATA;
    let sl = T::__DATA;
    println!("{:?}", t);
    println!("{:?}", sl);
}

pub fn tester_2(test_setting_manual: TestSettingManual) {
    let t = test_setting_manual.test_data;
    println!("{:?}", t);
    println!("{:?}", test_setting_manual);
}
