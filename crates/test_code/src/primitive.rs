#[cfg(test)]
mod test {
    use const_struct::{
        const_struct,
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

    pub const fn tester_option_option<T: OptionTy<Option<f32>>>() -> Option<Option<f32>> {
        T::VALUE
    }

    pub const fn tester_test_setting<T: OptionTy<TestSetting>>() -> Option<TestSetting> {
        T::VALUE
    }

    #[const_struct]
    const PI: f32 = 3.1415926;

    #[const_struct]
    const SOME_PI: Option<f32> = Some(3.1415926);

    #[test]
    pub fn call_tester() {
        assert_eq!(tester_inner::<F32!(-0.5)>(), -0.5);
        assert_eq!(tester_inner::<F32!(-25.333)>(), -25.333);
        assert_eq!(tester_inner_u32::<U32!(25)>(), 25);
        assert_eq!(tester_inner_option::<Some!(F32!(-25.333))>(), Some(-25.333));
        assert_eq!(tester_inner_option::<None!()>(), None);
        assert_eq!(tester_inner::<PiTy>(), 3.1415926);
        assert_eq!(tester_inner_option::<Some!(PiTy)>(), Some(3.1415926));
        assert_eq!(tester_inner_option::<None!()>(), None);
        assert_eq!(tester_inner_option::<SomePiTy>(), Some(3.1415926));
        assert_eq!(tester_option_option::<Some!(Some!(PiTy))>(), Some(Some(3.1415926)));
        assert_eq!(tester_option_option::<Some!(None!())>(), Some(None));
        assert_eq!(tester_option_option::<None!()>(), None);
        assert_eq!(tester_option_option::<Some!(SomePiTy)>(), Some(Some(3.1415926)));

        // debug_assert_eq!(tester_test_setting::<Some!(TestSetting::default())>(), Some(TestSetting::default()));
    }
}
