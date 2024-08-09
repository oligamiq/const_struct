use const_struct::{
    primitive::{F32Ty, OptionTy, U32Ty},
    None, Some, F32, U32,
};

use crate::setting::TestSetting;

pub const fn tester_inner<T: F32Ty>() -> f32 {
    T::VALUE
}

pub const fn tester_inner_u32<T: U32Ty>() -> u32 {
    T::VALUE
}

pub const fn tester_inner_option<T: OptionTy<f32>>() -> Option<f32> {
    T::VALUE
}

pub const fn tester_test_setting<T: OptionTy<TestSetting>>() -> Option<TestSetting> {
    T::VALUE
}

#[test]
pub fn call_tester() {
    debug_assert_eq!(tester_inner::<F32!(-0.5)>(), -0.5);
    debug_assert_eq!(tester_inner::<F32!(-25.333)>(), -25.333);
    debug_assert_eq!(tester_inner_u32::<U32!(25)>(), 25);
    debug_assert_eq!(tester_inner_option::<Some!(F32!(-25.333))>(), Some(-25.333));
    debug_assert_eq!(tester_inner_option::<None!()>(), None);

    // debug_assert_eq!(tester_test_setting::<Some!(TestSetting::default())>(), Some(TestSetting::default()));
}
