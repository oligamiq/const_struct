use std::fmt::Debug;

use crate::{
    pre::ConstStructTraits,
    primitive::some::PrimitiveTraits,
    struct_prim::{ConstStructPrimQueue, ConstStructPrimData, ConstStructPrimEnd},
    F32,
};

pub trait Float {}

impl Float for f32 {}

#[derive(Debug)]
pub struct TestGenerics<S: Float> {
    s: S,
}

pub trait TestGenericsTy<S: Float + Copy>: ConstStructTraits<TestGenerics<S>> {
    const S: S = Self::__DATA.s;
}

impl<S: Float + Copy, U: ConstStructTraits<TestGenerics<S>>> TestGenericsTy<S> for U {}

type TestGenericsPrimWrapper<S, A> =
    ConstStructPrimQueue<TestGenerics<S>, ConstStructPrimQueue<A, ConstStructPrimEnd>>;

impl<S: Float + Copy, A: ConstStructPrimData<Data = S>> PrimitiveTraits
    for TestGenericsPrimWrapper<S, A>
{
    type DATATYPE = TestGenerics<S>;
    const __DATA: Self::DATATYPE =
        <TestGenericsPrimWrapper<S, A> as ConstStructTraits<TestGenerics<S>>>::__DATA;
}

impl<S: Float + Copy, A: ConstStructPrimData<Data = S>> ConstStructTraits<TestGenerics<S>>
    for TestGenericsPrimWrapper<S, A>
{
    const __DATA: TestGenerics<S> = TestGenerics { s: A::__DATA };
}

macro_rules! TestGenerics {
    ($s:expr, $a:expr) => {
        paste::paste! {
            ConstStructPrimQueue<TestGenerics<$s>, ConstStructPrimQueue<
                [<$s:camel>]!($a.s),
                ConstStructPrimEnd,
            >>
        }
    };
}

#[test]
fn call_macro() {
    call_tester::<f32, TestGenerics!(f32, TestGenerics { s: 0.6 })>();
}

fn call_tester<S: Debug + Copy + Float, T: TestGenericsTy<S>>() {
    println!("{:?}", T::__DATA);
}
