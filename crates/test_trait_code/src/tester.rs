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


// trait F32Ty {
//     const __DATA: f32;
// }

// macro_rules! F32 {
//     ($num: literal) => {
//         struct F32TyImpl;

//         impl F32Ty for F32TyImpl {
//             const __DATA: f32 = $num;
//         }
//     };
// }

// struct F32_3_0;

// impl F32Ty for F32_3_0 {
//     const __DATA: f32 = 3 as f32;
// }

// const fn f32_to_u32(f: f32) -> u32 {
//     unsafe { std::mem::transmute(f) }
// }

// const fn u32_to_f32(i: u32) -> f32 {
//     unsafe { std::mem::transmute(i) }
// }

// type F32Ty = u32;

// fn tester_inner_f<const N: F32Ty>() {
//     println!("{:?}", N);
// }

// fn tester_f() {
//     tester_inner_f::<{
//         f32_to_u32(3.0)
//     }>();
// }


