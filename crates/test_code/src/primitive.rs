#[cfg(test)]
mod test {
    use const_struct::{
        const_struct,
        primitive::{F32Ty, OptionTy, TupleTy, U32Ty},
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

    pub const fn tester_option_option<T: OptionTy<Option<f32>>>() -> Option<Option<f32>> {
        T::VALUE
    }

    pub const fn tester_option_option_test_setting<T: OptionTy<Option<TestSetting>>>() -> Option<Option<TestSetting>> {
        T::VALUE
    }

    pub const fn tester_tuple<T: TupleTy<(f32,)>>() -> (f32,) {
        T::VALUE
    }

    pub const fn tester_tuple2<T: TupleTy<(f32, f32)>>() -> (f32, f32) {
        T::VALUE
    }

    pub const fn tester_tuple_tuple<T: TupleTy<(f32, (f32, f32))>>() -> (f32, (f32, f32)) {
        T::VALUE
    }

    #[const_struct]
    const PI: f32 = 3.1415926;

    #[const_struct]
    const SOME_PI: Option<f32> = Some(3.1415926);

    #[const_struct]
    const SOME_PI_TY: Option<f32> = Some(PI);

    #[const_struct]
    const TEST_SETTING_A: TestSetting = TestSetting::default();

    #[const_struct]
    const SOME_TEST_SETTING_A: Option<TestSetting> = Some(TestSetting::default());

    #[const_struct]
    const TUPLE_F32: (f32,) = (PI, );

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
        assert_eq!(tester_inner_option::<None!()>(), None);
        assert_eq!(tester_test_setting::<Some!(TestSettingATy)>(), Some(TestSetting::default()));
        assert_eq!(tester_test_setting::<None!()>(), None);
        assert_eq!(tester_test_setting::<SomeTestSettingATy>(), Some(TestSetting::default()));
        assert_eq!(tester_option_option::<Some!(Some!(F32!(-25.333)))>(), Some(Some(-25.333)));
        assert_eq!(tester_option_option::<Some!(SomePiTy)>(), Some(Some(3.1415926)));
        assert_eq!(tester_option_option::<None!()>(), None);
        assert_eq!(tester_option_option_test_setting::<Some!(Some!(TestSettingATy))>(), Some(Some(TestSetting::default())));
        assert_eq!(tester_option_option_test_setting::<Some!(SomeTestSettingATy)>(), Some(Some(TestSetting::default())));
        assert_eq!(tester_tuple::<(F32!(-0.5), )>(), (-0.5, ));
        assert_eq!(tester_tuple::<(PiTy, )>(), (3.1415926, ));
        assert_eq!(tester_tuple::<TupleF32Ty>(), (3.1415926, ));
        assert_eq!(tester_tuple2::<(F32!(-0.5), F32!(-25.333))>(), (-0.5, -25.333));
        assert_eq!(tester_tuple2::<(PiTy, PiTy)>(), (3.1415926, 3.1415926));
        assert_eq!(tester_tuple2::<(F32!(-0.5), PiTy)>(), (-0.5, 3.1415926));
        assert_eq!(tester_tuple_tuple::<(F32!(-0.5), (F32!(-25.333), F32!(0.0)))>(), (-0.5, (-25.333, 0.0)));

        // assert_eq!(tester_test_setting::<Some!(TestSetting::default())>(), Some(TestSetting::default()));
    }
}
