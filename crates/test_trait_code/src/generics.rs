#[derive(Debug)]
pub struct TestSettingManual<const T: usize> {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: [u8; T],
    str: &'static str,
}
