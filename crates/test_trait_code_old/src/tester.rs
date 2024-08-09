use crate::TestSettingManualTy;

pub fn tester<T: TestSettingManualTy>() {
    let t = T::TEST_DATA;
    println!("{:?}", t);
}
