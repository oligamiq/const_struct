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

pub struct Tester<T: TestSettingManualTy> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: TestSettingManualTy> Tester<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn tester(&self) {
        let t = T::TEST_DATA;
        println!("{:?}", t);
    }
}

pub struct Tester2 {
    test_setting_manual: TestSettingManual,
}

impl Tester2 {
    pub fn new(test_setting_manual: TestSettingManual) -> Self {
        Self { test_setting_manual }
    }

    pub fn tester(&self) {
        let t = self.test_setting_manual.test_data;
        println!("{:?}", t);
    }
}
