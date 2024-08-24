#[derive(Debug)]
pub struct TestStruct {
    test_data: [u8; 16],
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    str: &'static str,
}
