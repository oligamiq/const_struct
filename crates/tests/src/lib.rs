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
    use const_struct::{const_struct, primitive::F32Ty};

    pub fn tester<A: F32Ty>() {
        println!("a: {:?}", A::__DATA);
    }

    #[const_struct]
    const PI: f32 = 3.14159265358979;

    #[test]
    fn main() {
        tester::<PiTy>();
    }
}
