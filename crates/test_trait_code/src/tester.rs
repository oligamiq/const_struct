use crate::TestSettingManualTy;

pub fn tester<T: TestSettingManualTy>() {
    let t = T::TEST_DATA;
    let sl = T::__DATA;
    println!("{:?}", t);
    println!("{:?}", sl);
}
