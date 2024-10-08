#[cfg(test)]
mod tests1 {
    use const_struct::{primitive::F32Ty, F32};

    pub fn tester<A: F32Ty>() {
        println!("a: {:?}", A::__DATA);
    }

    #[test]
    fn main() {
        tester::<F32!(0.5)>();
    }
}

#[cfg(test)]
mod tests2 {
    use const_struct::{const_struct, ConstStruct};

    #[derive(ConstStruct, Debug)]
    pub struct TestSetting {
        pub a: Option<u32>,
        abc_def: &'static str,
    }

    pub fn tester<A: TestSettingTy>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const WINDOW_SETTING: TestSetting = {
        let mut c = TestSetting {
            a: Some(0),
            abc_def: "abc_def",
        };
        c.a = Some(5);
        c.abc_def = "hello world";
        c
    };

    #[test]
    fn main() {
        tester::<WindowSettingTy>();
    }
}

#[cfg(test)]
mod tests3 {
    use const_struct::{primitive::OptionTy, Some, F32};

    pub fn tester<A: OptionTy<Option<f32>>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[test]
    fn main() {
        tester::<Some!(Some!(F32!(0.5)))>();
    }
}

#[cfg(test)]
mod tests4 {
    use const_struct::{primitive::TupleTy, F32, F64, U32};

    pub fn tester<A: TupleTy<(f32, f64, u32)>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[test]
    fn main() {
        tester::<(F32!(0.5), F64!(0.5), U32!(0))>();
    }
}

#[cfg(test)]
mod tests5 {
    use const_struct::{const_struct, primitive::F64Ty};

    pub fn tester<A: F64Ty>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const PI: f64 = 3.14159265358979;

    #[test]
    fn main() {
        tester::<PiTy>();
    }
}

#[cfg(test)]
mod tests6 {
    use const_struct::{const_struct, primitive::OptionTy};

    pub fn tester<A: OptionTy<f64>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const PI: Option<f64> = Some(3.14159265358979);

    #[test]
    fn main() {
        tester::<PiTy>();
    }
}

#[cfg(test)]
mod test7 {
    #[allow(unused)]
    #[const_struct::const_struct(macro_export)]
    #[derive(const_struct::ConstStruct, Debug)]
    pub struct TestSetting {
        a: Option<u32>,
        abc_def: &'static str,
    }

    pub fn tester<A: TestSettingTy>() {
        println!("a: {:?}", A::__DATA);
    }

    pub const fn default() -> TestSetting {
        TestSetting {
            a: None,
            abc_def: "hello world",
        }
    }

    #[test]
    fn main() {
        tester::<macros::TestSetting!(default())>();
    }
}

#[cfg(test)]
pub mod test8 {
    use const_struct::{call_with_generics, ConstStruct};
    use core::fmt::Debug;

    pub trait Float {}

    impl Float for f32 {}
    impl Float for f64 {}

    #[const_struct::const_struct(Float: crate::test8::Float)]
    #[derive(ConstStruct, Debug)]
    pub struct TestSetting<S: Float> {
        pub a: Option<S>,
        abc_def: &'static str,
    }

    pub fn tester<A: TestSettingTy<f32>>() {
        println!("a: {:?}", A::__DATA);
    }

    pub fn tester2<S: Float + Debug + Copy, A: TestSettingTy<S>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[test]
    fn main() {
        tester::<
            macros::TestSetting!(
                f32,
                TestSetting {
                    a: None,
                    abc_def: "hello world"
                }
            ),
        >();
        tester2::<
            f32,
            macros::TestSetting!(
                f32,
                TestSetting {
                    a: None,
                    abc_def: "hello world"
                }
            ),
        >();
        call_with_generics!(tester2::<
            macros::TestSetting!(
                f64,
                TestSetting {
                    a: None,
                    abc_def: "hello world"
                }
            ),
        >());
    }
}

#[cfg(test)]
mod test9 {
    use const_struct::{call_with_generics, const_struct, ConstStruct};

    #[derive(ConstStruct, Debug)]
    pub struct TestSetting<const N: usize>;

    pub fn tester<const N: usize, A: TestSettingTy<N>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const B: TestSetting<5> = TestSetting;

    #[test]
    fn main() {
        tester::<5, macros::TestSetting!(5, TestSetting::<5>)>();
        tester::<5, macros::TestSetting!(_, TestSetting::<5>)>();
        tester::<4, macros::TestSetting!(4, TestSetting)>();
        tester::<9, macros::TestSetting!(TestSetting::<9>)>();

        tester::<5, macros::TestSetting!(B)>();
        tester::<5, BTy>();
        call_with_generics!(tester::<macros::TestSetting!(B)>());
        call_with_generics!(tester::<5, BTy>());
        call_with_generics!(tester::<macros::TestSetting!(_, BTy)>());
        call_with_generics!(tester::<macros::TestSetting!(BTy)>());
    }
}

#[cfg(test)]
mod test10 {
    use const_struct::{primitive::TupleTy, ConstStruct, F32};

    #[derive(ConstStruct, Debug)]
    pub struct TestSetting;

    pub fn tester<A: TupleTy<(f32, TestSetting)>>() {
        println!("a: {:?}", A::__DATA);
    }

    // pub mod ttt {
    //     macro_rules! say_hello {
    //         // `()` indicates that the macro takes no argument.
    //         // `()`はマクロが引数をとらないことを示す。
    //         () => {
    //             // The macro will expand into the contents of this block.
    //             // マクロは（訳注: プリコンパイルの段階で）このブロック内の内容に展開されます。
    //             println!("Hello!")
    //         };
    //     }

    //     pub(crate) use say_hello;
    // }

    #[test]
    fn main() {
        // let tt = ttt::say_hello!();
        tester::<(F32!(0.5), macros::TestSetting!(TestSetting))>();
    }
}

#[cfg(test)]
mod test11 {
    use const_struct::{call_with_generics, const_struct, primitive::TupleTy, ConstStruct, F32};

    #[derive(ConstStruct, Debug)]
    pub struct TestSetting<const N: usize>;

    pub fn tester<const N: usize, A: TupleTy<(f32, TestSetting<N>)>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const B: TestSetting<0> = TestSetting;

    #[test]
    fn main() {
        tester::<0, (F32!(0.5), BTy)>();
        call_with_generics!(tester::<(F32!(0.5), macros::TestSetting!(BTy))>());
    }
}

#[cfg(test)]
mod test12 {
    use const_struct::{const_struct, primitive::TupleTy, PrimitiveTraits};

    #[derive(Debug)]
    pub struct TestSetting;

    pub fn tester<A: TupleTy<(TestSetting,)>>() {
        println!("a: {:?}", A::__DATA);
    }

    pub fn tester_alt<A: PrimitiveTraits<DATATYPE = TestSetting>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const B: TestSetting = TestSetting;

    #[test]
    fn main() {
        tester::<(BTy,)>();
        tester_alt::<BTy>();
    }
}

#[cfg(test)]
mod test13 {
    use const_struct::{const_struct, primitive::TupleTy};

    pub trait Float {}

    impl Float for f32 {}

    #[derive(Debug)]
    pub struct TestSetting<F: Float> {
        _a: F,
    }

    pub fn tester<F: Float + core::fmt::Debug + Copy, A: TupleTy<(TestSetting<F>,)>>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const B: TestSetting<f32> = TestSetting { _a: 0.5 };

    #[test]
    fn main() {
        tester::<f32, (BTy,)>();
    }
}

#[cfg(test)]
pub mod test14 {
    use const_struct::{const_struct, ConstStruct};

    #[derive(ConstStruct, Debug)]
    #[const_struct(TestSettingB: crate::test14::TestSettingB)]
    pub struct TestSettingB;

    pub fn tester<A: TestSettingBTy>() {
        println!("a: {:?}", A::__DATA);
    }

    pub mod module {
        #[test]
        fn main() {
            super::tester::<super::macros::TestSettingB!(super::TestSettingB)>();
        }
    }
}

#[cfg(test)]
pub mod test15 {
    use const_struct::{const_struct, ConstStruct};
    use core::fmt::Debug;

    #[derive(Debug, Copy, Clone)]
    pub struct Float32;

    pub trait Float {}

    impl Float for Float32 {}

    #[derive(ConstStruct, Debug)]
    #[const_struct(
        TestSettingC: crate::test15::TestSettingC,
    )]
    pub struct TestSettingC<const N: usize, F: Float> {
        _a: F,
    }

    pub fn tester<const N: usize, F: Float + Copy + Debug, A: TestSettingCTy<N, F>>() {
        println!("a: {:?}", A::__DATA);
    }

    pub mod module {
        #[test]
        fn main() {
            const_struct::call_with_generics!(super::tester::<
                super::macros::TestSettingC!(
                    super::Float32,
                    super::TestSettingC::<7, super::Float32> { _a: super::Float32 }
                ),
            >());
        }
    }
}

const_struct::init!();
