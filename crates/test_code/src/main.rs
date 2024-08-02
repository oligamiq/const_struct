use std::marker::PhantomData;

use const_struct_derive::ConstStruct;

mod setting;

fn main() {}

#[derive(ConstStruct)]
struct TestSetting {
    a: Option<u32>,
}

impl TestSetting {
    pub const fn default() -> Self {
        Self { a: None }
    }
}
