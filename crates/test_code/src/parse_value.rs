#[cfg(test)]
mod test {
    pub struct TestGenerics<const A: usize> {
        pub a: usize,
    }
    use const_struct::match_underscore;
    use const_struct::struct_prim::{ConstStructPrimQueue, ConstStructPrimEnd};
    use const_struct::Usize;

    macro_rules! TestGenerics {
        (@TestGenericsGetGenericsData, $macro_path: path, $($arg:tt)*) => {
            $macro_path!(@TestGenericsGetGenericsData(const), $($arg)*)
        };
        (@TestGenericsGetInnerGenerics0, $value:expr) => {
            {
                const fn get_const_generics_a<const A: usize>(_: TestGenerics<A>) -> usize {
                    A
                }

                get_const_generics_a($value)
            }
        };
        ($a:tt, $value:expr) => {
            ConstStructPrimQueue<TestGenerics<{
                match_underscore!($a, {
                    const fn get_const_generics_a<const A: usize>(_: TestGenerics<A>) -> usize {
                        A
                    }

                    get_const_generics_a($value)
                })
            }>, ConstStructPrimQueue<
                Usize!({
                    let value: TestGenerics<{
                        match_underscore!($a, {
                            const fn get_const_generics_a<const A: usize>(value: TestGenerics<A>) -> usize {
                                A
                            }

                            get_const_generics_a($value)
                        })
                    }> = $value;
                    value
                }.a)
            , ConstStructPrimEnd>>
        };
    }

    #[test]
    pub fn test() {
        // type A = const_struct::parse_value!((f32, u32), (0.6, 4));
        // no_std_compat::println!("{:?}", <A as ::const_struct::PrimitiveTraits>::__DATA);

        // type B = const_struct::parse_value!(Option<Option<f32>>, Some(Some(0.6)));

        type B = const_struct::parse_value!(
            @AdditionData(),
            (Option<Option<f32>>, f32, Option<bool>),
            (None, 0.6, Some(true))
        );

        assert_eq!(
            <B as ::const_struct::PrimitiveTraits>::__DATA,
            (None, 0.6, Some(true))
        );

        // no_std_compat::println!("{:?}", <B as ::const_struct::PrimitiveTraits>::__DATA);

        // type C = const_struct::parse_value!((TestGenerics<_>,), (TestGenerics { a: 7 },));
        // type C = const_struct::parse_value!(TestGenerics<7>, TestGenerics { a: 7 });
        // let t: C;

        // type C = (
        //     TestGenerics!({
        //         {
        //             let v0: (TestGenerics,) = (TestGenerics::<7>,);
        //             v0
        //         }
        //         .0
        //     }),
        // );

        // const_struct::parse_value!(
        //     ConstStructPrimQueue<
        //         TestGenerics<
        //             {
        //                 match_underscore!($a, {
        //                     const fn get_const_generics_a<const A: usize, S: Float + Copy>(_: TestGenerics<A, S>) -> usize {
        //                         A
        //                     }

        //                     get_const_generics_a($value)
        //                 })
        //             },
        //             seal!($s),
        //         >,
        //         f32,
        //     >,
        //     { unsafe { core::mem::zeroed() } }
        // )

        // type D = (
        //     TestGenerics!(_, {
        //         {
        //             let v0: (TestGenerics<_>,) = (TestGenerics { a: 7 },);
        //             v0
        //         }
        //         .0
        //     }),
        // );
    }
}
