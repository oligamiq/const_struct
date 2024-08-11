use std::fmt::Debug;

use crate::{
    pre::ConstStructTraits,
    primitive::{some::PrimitiveTraits, F32Ty},
    struct_prim::{ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd},
    F32,
};

pub trait Float {}

impl Float for f32 {}

#[derive(Debug)]
pub struct TestGenerics<const A: usize, S: Float> {
    s: S,
    t: [u8; A],
}

pub trait TestGenericsTy<const A: usize, S: Float + Copy>:
    ConstStructTraits<TestGenerics<A, S>>
{
    const S: S = Self::__DATA.s;
}

impl<const A: usize, S: Float + Copy, U: ConstStructTraits<TestGenerics<A, S>>> TestGenericsTy<A, S>
    for U
{
}

type TestGenericsPrimWrapper<const A: usize, S, TestGenericsS> =
    ConstStructPrimAny<TestGenerics<A, S>, ConstStructPrimAny<TestGenericsS, ConstStructPrimEnd>>;

impl<const A: usize, S: Float + Copy, TestGenericsS: ConstStructPrimData<Data = S>> PrimitiveTraits
    for TestGenericsPrimWrapper<A, S, TestGenericsS>
{
    type DATATYPE = TestGenerics<A, S>;
    const __DATA: Self::DATATYPE =
        <TestGenericsPrimWrapper<A, S, TestGenericsS> as ConstStructTraits<TestGenerics<A, S>>>::__DATA;
}

impl<const A: usize, S: Float + Copy, TestGenericsS: ConstStructPrimData<Data = S>>
    ConstStructTraits<TestGenerics<A, S>> for TestGenericsPrimWrapper<A, S, TestGenericsS>
{
    const __DATA: TestGenerics<A, S> = TestGenerics {
        s: TestGenericsS::__DATA,
        t: [0; A],
    };
}

macro_rules! TestGenerics {
    ($a:tt, $s:expr, $value:expr) => {
        paste::paste! {
            ConstStructPrimAny<TestGenerics<{
                const_struct_derive::match_underscore!($a, {
                    const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
                        A
                    }

                    get_const_generics_a($value)
                })
            }, $s>, ConstStructPrimAny<
                [<$s:camel>]!({
                    let value: TestGenerics<{
                        const_struct_derive::match_underscore!($a, {
                            const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
                                A
                            }

                            get_const_generics_a($value)
                        })
                    }, $s> = $value;
                    value
                }.s),
                ConstStructPrimEnd,
            >>
        }
    };
}

#[test]
fn call_macro() {
    call_tester::<56, f32, TestGenerics!(56, f32, TestGenerics { s: 0.6, t: [0; 56] })>();
    call_tester::<20, f32, TestGenerics!(_, f32, TestGenerics { s: 0.6, t: [0; 20] })>();
}

fn call_tester<const A: usize, S: Debug + Copy + Float, T: TestGenericsTy<A, S>>() {
    println!("{:?}", T::__DATA);
    println!("{:?}", A);
}
