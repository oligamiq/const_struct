use std::mem::transmute;

pub trait F32Ty<const N: u32> {
    const __DATA: f32 = unsafe { transmute(N) };
}

impl F32Ty<{ unsafe { transmute(5.0f32) } }> for f32 {}

pub fn tester<T: F32Ty<U>, const U: u32>() {
    println!("data: {}", T::__DATA);
}
