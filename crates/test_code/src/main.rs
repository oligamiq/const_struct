use const_struct_derive::ConstStruct;

mod setting;

fn main() {
}

#[derive(ConstStruct)]
struct TestSetting {
    a: Option<u32>,
}

impl TestSetting {
    pub const fn default() -> Self {
        Self { a: None }
    }
}


struct TestSettingManual {
    test_data: Option<u32>,
}

impl TestSettingManual {
}

trait TestSettingManualTyImpl {
    const __A: Option<u32>;
}

trait TestSettingManualTyImplData {
    type __A;
}

impl TestSettingManualTyImplData for TestSettingManual {
    type __A = Option<u32>;
}

trait TestSettingManualTy: TestSettingManualTyImpl {
    const TEST_DATA: Option<u32> = Self::__A;
}
