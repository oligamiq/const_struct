use std::mem::transmute;

pub trait PrimTy {
    const __DATA: Self;
}

// impl F32Ty for f32 {
//     const __DATA: f32;
// }

// pub fn tester<T: F32Ty<U>, const U: u32>() {
//     println!("data: {}", T::__DATA);
// }
