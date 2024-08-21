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

impl Float for (f32, u32) {}

#[derive(Debug)]
pub struct TestGenerics<const A: usize, S: Float> {
    s: S,
    t: [u8; A],
}

impl<const A: usize, S: Float> KeepType<0> for TestGenerics<A, S> {
    type Type = usize;
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

impl<const A: usize, S: Float + Copy, TestGenericsS: PrimitiveTraits<DATATYPE = S>> PrimitiveTraits
    for TestGenericsPrimWrapper<A, S, TestGenericsS>
{
    type DATATYPE = TestGenerics<A, S>;
    const __DATA: Self::DATATYPE =
        <TestGenericsPrimWrapper<A, S, TestGenericsS> as ConstStructTraits<TestGenerics<A, S>>>::__DATA;
}

impl<const A: usize, S: Float + Copy, TestGenericsS: PrimitiveTraits<DATATYPE = S>>
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
        (@TestGenericsGetGenericsData, $macro_path: path, $($arg:tt)*) => {
            $macro_path!(@TestGenericsGetGenericsData(
                const fn get_const_generics<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) {},
                parse_value!(
                    @AdditionData(F32: F32),
                    gen!(S),
                    { expr!() }.s
                ),
                parse_value!(
                    @AdditionData(F32: F32),
                    [u8; gen!(A)],
                    { expr!($value) }.t
                )
            ), $($arg)*)
        };
        (@TestGenericsGetInnerGenerics0, $value:expr) => {
            {
                const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
                    A
                }

                get_const_generics_a($value)
            }
        };
        (@TestGenericsGetInnerGenerics1, $value:expr) => {
            {
                panic!("cannot use _ in this context")
            }
        };
        ($a:tt, $s:tt, $value:expr) => {
            paste::paste! {
                ConstStructPrimAny<TestGenerics<{
                    match_underscore!($a, {
                        const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
                            A
                        }

                        get_const_generics_a($value)
                    })
                }, $s>, ConstStructPrimAny<
                    // @AdditionData(F32: F32)を用いて、const_structのF32マクロではなく、このライブラリのF32マクロを使う
                    const_struct::parse_value!(@AdditionData(F32: F32), $s, {
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
    // call_with_generics!(call_tester::<
    //     7,
    //     crate::TestGenerics!(
    //         _,
    //         (f32, u32),
    //         TestGenerics {
    //             s: (0.6, 4),
    //             t: [0; 56]
    //         }
    //     ),
    //     9,
    // >());
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

#[automatically_derived]
impl PrimitiveTraits for BTy {
    type DATATYPE = TestGenerics<7, f32>;
    const __DATA: <Self as PrimitiveTraits>::DATATYPE = B;
}

pub trait KeepTypeConst<const N: usize> {
    type DATATYPE;
    const N: Self::DATATYPE;
}

pub trait KeepType<const N: usize> {
    type Type;
}

impl KeepTypeConst<0> for BTy {
    type DATATYPE = <TestGenerics<7, f32> as KeepType<0>>::Type;
    const N: Self::DATATYPE = 7;
}

impl KeepType<1> for BTy {
    type Type = f32;
}

#[test]
fn test_test_generics() {
    call_with_generics!(call_tester::<4, crate::TestGenerics!(_, _, BTy), 9>());
    call_with_generics!(call_tester::<4, crate::TestGenerics!(BTy), 9>());
}
