use std::fmt::Debug;

use crate::{
    pre::ConstStructTraits,
    primitive::{some::PrimitiveTraits, F32Ty},
    struct_prim::{ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd},
    F32,
};

use const_struct_derive::{call_with_generics, const_struct};

pub trait Float {}

impl Float for f32 {}

#[derive(Debug)]
pub struct TestGenerics<const A: usize, S: Float> {
    s: S,
    t: [u8; A],
}

pub trait TestGenericsTy<const A: usize, S: Float + Copy>:
    PrimitiveTraits<DATATYPE = TestGenerics<A, S>>
{
    const S: S = <Self as PrimitiveTraits>::__DATA.s;
}

impl<const A: usize, S: Float + Copy, U: PrimitiveTraits<DATATYPE = TestGenerics<A, S>>>
    TestGenericsTy<A, S> for U
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

macro_rules! match_underscore {
    (_, $tt_is_underscore:expr) => {
        $tt_is_underscore
    };
    ($input:expr, $tt_is_underscore:expr) => {
        $input
    };
}

pub mod tt {
    use crate::{
        pre::ConstStructTraits,
        primitive::{some::PrimitiveTraits, F32Ty},
        struct_prim::{ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd},
        F32,
    };

    #[macro_export]
    macro_rules! TestGenerics {
        (TestGenericsGetGenericsData, $macro_path: path, $($arg:tt)*) => {
            $macro_path!(TestGenericsGetGenericsData(const, type), $($arg)*)
        };
        (TestGenericsGetInnerGenerics0, $value:expr) => {
            {
                const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
                    A
                }

                get_const_generics_a($value)
            }
        };
        (TestGenericsGetInnerGenerics1, $value:expr) => {
            {
                panic!("cannot use _ in this context")
            }
        };
        ($a:tt, $s:expr, $value:expr) => {
            paste::paste! {
                ConstStructPrimAny<TestGenerics<{
                    match_underscore!($a, {
                        const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
                            A
                        }

                        get_const_generics_a($value)
                    })
                }, $s>, ConstStructPrimAny<
                    [<$s:camel>]!({
                        let value: TestGenerics<{
                            match_underscore!($a, {
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
}

#[test]
fn call_macro() {
    call_with_generics!(call_tester::<
        7,
        crate::TestGenerics!(_, f32, TestGenerics { s: 0.6, t: [0; 56] }),
        9,
    >());
}

fn call_tester<
    const C: usize,
    const A: usize,
    S: Debug + Copy + Float,
    T: TestGenericsTy<A, S>,
    const U: usize,
>() {
    println!("{:?}", T::__DATA);
    println!("{:?}", A);
    println!("{:?}", T::S);
    println!("{:?}", C);
    println!("{:?}", U);
}

const B: TestGenerics<7, f32> = TestGenerics { s: 0.0, t: [0; 7] };

#[automatically_derived]
pub struct BTy;

pub trait KeepType {
    type Type;
}

pub struct KeepTypeStruct<T, const N: usize> {
    __phantom: core::marker::PhantomData<T>,
}

pub struct KeepTypeConst<const N: usize>;

impl<const N: usize> KeepTypeConst<N> {
    pub const __DATA: usize = N;
}

impl KeepType for KeepTypeStruct<BTy, 0> {
    type Type = KeepTypeConst<7>;
}

impl KeepType for KeepTypeStruct<BTy, 1> {
    type Type = f32;
}

#[automatically_derived]
impl PrimitiveTraits for BTy {
    type DATATYPE = TestGenerics<7, f32>;
    const __DATA: <Self as PrimitiveTraits>::DATATYPE = B;
}

#[test]
fn test_test_generics() {
    call_with_generics!(call_tester::<4, crate::TestGenerics!(_, _, BTy), 9>());
    call_with_generics!(call_tester::<4, crate::TestGenerics!(BTy), 9>());
}
