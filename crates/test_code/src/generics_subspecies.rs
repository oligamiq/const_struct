
use const_struct::{call_with_generics, const_struct, ConstStruct};
#[derive(ConstStruct, Debug)]
pub struct TestGenerics<const T: usize> {
    float: f32,
}
const COUNT: usize = 7;
#[const_struct]
const B: TestGenerics<{ COUNT }> = TestGenerics { float: 0.0 };

const fn constant<const T: usize, S: TestGenericsTy<T>>() -> f32 {
    S::FLOAT
}

const FLOAT: f32 = call_with_generics!(constant::<test_generics!(BTy)>());
