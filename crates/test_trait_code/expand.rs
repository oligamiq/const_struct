mac: {
    crate :: TestGenerics!
    (TestGenericsGetConstGenerics0, TestGenerics { s : 0.6, t : [0; 56] })
}
str: f32
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use core::str;
use primitive::{
    some::{OptionTy, PrimitiveTraits},
    F32Ty,
};
use setting::WINDOW_SETTING_MANUAL;
use struct_prim::{
    ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd, ConstStructPrimOption,
    ConstStructPrimU32, ConstStructPrimU8Vec, ConstStructPrimU8VecLimit,
};
use tester::{tester, tester_2};
mod generics {
    use std::fmt::Debug;
    use crate::{
        pre::ConstStructTraits, primitive::some::PrimitiveTraits,
        struct_prim::{ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd},
        F32,
    };
    pub trait Float {}
    impl Float for f32 {}
    pub struct TestGenerics<S: Float> {
        s: S,
    }
    #[automatically_derived]
    impl<S: ::core::fmt::Debug + Float> ::core::fmt::Debug for TestGenerics<S> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TestGenerics",
                "s",
                &&self.s,
            )
        }
    }
    pub trait TestGenericsTy<S: Float + Copy>: ConstStructTraits<TestGenerics<S>> {
        const S: S = Self::__DATA.s;
    }
    impl<S: Float + Copy, U: ConstStructTraits<TestGenerics<S>>> TestGenericsTy<S>
    for U {}
    type TestGenericsPrimWrapper<S, A> = ConstStructPrimAny<
        TestGenerics<S>,
        ConstStructPrimAny<A, ConstStructPrimEnd>,
    >;
    impl<S: Float + Copy, A: ConstStructPrimData<Data = S>> PrimitiveTraits
    for TestGenericsPrimWrapper<S, A> {
        type DATATYPE = TestGenerics<S>;
        const __DATA: Self::DATATYPE = <TestGenericsPrimWrapper<
            S,
            A,
        > as ConstStructTraits<TestGenerics<S>>>::__DATA;
    }
    impl<
        S: Float + Copy,
        A: ConstStructPrimData<Data = S>,
    > ConstStructTraits<TestGenerics<S>> for TestGenericsPrimWrapper<S, A> {
        const __DATA: TestGenerics<S> = TestGenerics { s: A::__DATA };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "generics::call_macro"]
    pub const call_macro: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("generics::call_macro"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "crates/test_trait_code/src/generics.rs",
            start_line: 54usize,
            start_col: 4usize,
            end_line: 54usize,
            end_col: 14usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(call_macro()),
        ),
    };
    fn call_macro() {
        call_tester::<
            f32,
            ConstStructPrimAny<
                TestGenerics<f32>,
                ConstStructPrimAny<
                    crate::primitive::F32Impl<
                        {
                            unsafe {
                                core::mem::transmute::<f32, u32>(TestGenerics { s: 0.6 }.s)
                            }
                        },
                    >,
                    ConstStructPrimEnd,
                >,
            >,
        >();
    }
    fn call_tester<S: Debug + Copy + Float, T: TestGenericsTy<S>>() {
        {
            ::std::io::_print(format_args!("{0:?}\n", T::__DATA));
        };
    }
}
mod generics_with_const {
    use std::fmt::Debug;
    use crate::{
        pre::ConstStructTraits, primitive::{some::PrimitiveTraits, F32Ty},
        struct_prim::{ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd},
        F32,
    };
    use const_struct_derive::{call_with_generics, const_struct};
    pub trait Float {}
    impl Float for f32 {}
    pub struct TestGenerics<const A: usize, S: Float> {
        s: S,
        t: [u8; A],
    }
    #[automatically_derived]
    impl<const A: usize, S: ::core::fmt::Debug + Float> ::core::fmt::Debug
    for TestGenerics<A, S> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TestGenerics",
                "s",
                &self.s,
                "t",
                &&self.t,
            )
        }
    }
    pub trait TestGenericsTy<
        const A: usize,
        S: Float + Copy,
    >: PrimitiveTraits<DATATYPE = TestGenerics<A, S>> {
        const S: S = <Self as PrimitiveTraits>::__DATA.s;
    }
    impl<
        const A: usize,
        S: Float + Copy,
        U: PrimitiveTraits<DATATYPE = TestGenerics<A, S>>,
    > TestGenericsTy<A, S> for U {}
    type TestGenericsPrimWrapper<const A: usize, S, TestGenericsS> = ConstStructPrimAny<
        TestGenerics<A, S>,
        ConstStructPrimAny<TestGenericsS, ConstStructPrimEnd>,
    >;
    impl<
        const A: usize,
        S: Float + Copy,
        TestGenericsS: ConstStructPrimData<Data = S>,
    > PrimitiveTraits for TestGenericsPrimWrapper<A, S, TestGenericsS> {
        type DATATYPE = TestGenerics<A, S>;
        const __DATA: Self::DATATYPE = <TestGenericsPrimWrapper<
            A,
            S,
            TestGenericsS,
        > as ConstStructTraits<TestGenerics<A, S>>>::__DATA;
    }
    impl<
        const A: usize,
        S: Float + Copy,
        TestGenericsS: ConstStructPrimData<Data = S>,
    > ConstStructTraits<TestGenerics<A, S>>
    for TestGenericsPrimWrapper<A, S, TestGenericsS> {
        const __DATA: TestGenerics<A, S> = TestGenerics {
            s: TestGenericsS::__DATA,
            t: [0; A],
        };
    }
    pub mod tt {
        use crate::{
            pre::ConstStructTraits, primitive::{some::PrimitiveTraits, F32Ty},
            struct_prim::{ConstStructPrimAny, ConstStructPrimData, ConstStructPrimEnd},
            F32,
        };
        use const_struct_derive::call_with_generics;
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "generics_with_const::call_macro"]
    pub const call_macro: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("generics_with_const::call_macro"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "crates/test_trait_code/src/generics_with_const.rs",
            start_line: 127usize,
            start_col: 4usize,
            end_line: 127usize,
            end_col: 14usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(call_macro()),
        ),
    };
    fn call_macro() {
        call_tester::<
            7,
            {
                {
                    const fn get_const_generics_a<const A: usize, S: Float + Copy>(
                        _: TestGenerics<A, S>,
                    ) -> usize {
                        A
                    }
                    get_const_generics_a(TestGenerics { s: 0.6, t: [0; 56] })
                }
            },
            f32,
            ConstStructPrimAny<
                TestGenerics<
                    {
                        {
                            const fn get_const_generics_a<
                                const A: usize,
                                S: Float + Copy,
                            >(_: TestGenerics<A, S>) -> usize {
                                A
                            }
                            get_const_generics_a(TestGenerics { s: 0.6, t: [0; 56] })
                        }
                    },
                    f32,
                >,
                ConstStructPrimAny<
                    crate::primitive::F32Impl<
                        {
                            unsafe {
                                core::mem::transmute::<
                                    f32,
                                    u32,
                                >(
                                    {
                                        let value: TestGenerics<
                                            {
                                                {
                                                    const fn get_const_generics_a<
                                                        const A: usize,
                                                        S: Float + Copy,
                                                    >(_: TestGenerics<A, S>) -> usize {
                                                        A
                                                    }
                                                    get_const_generics_a(TestGenerics { s: 0.6, t: [0; 56] })
                                                }
                                            },
                                            f32,
                                        > = TestGenerics { s: 0.6, t: [0; 56] };
                                        value
                                    }
                                        .s,
                                )
                            }
                        },
                    >,
                    ConstStructPrimEnd,
                >,
            >,
            9,
        >();
    }
    fn call_tester<
        const C: usize,
        const A: usize,
        S: Debug + Copy + Float,
        T: TestGenericsTy<A, S>,
        const U: usize,
    >() {
        {
            ::std::io::_print(format_args!("{0:?}\n", T::__DATA));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", A));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", T::S));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", C));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", U));
        };
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
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "generics_with_const::test_test_generics"]
    pub const test_test_generics: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("generics_with_const::test_test_generics"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "crates/test_trait_code/src/generics_with_const.rs",
            start_line: 183usize,
            start_col: 4usize,
            end_line: 183usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_test_generics()),
        ),
    };
    fn test_test_generics() {
        (/*ERROR*/);
    }
}
mod pre {
    pub trait ConstStructTraits<T> {
        const __DATA: T;
    }
}
mod primitive {
    #![allow(unused)]
    use core::mem::transmute;
    use some::PrimitiveTraits;
    use crate::{pre::ConstStructTraits, struct_prim::ConstStructPrimData};
    pub trait F32Ty {
        const __DATA: f32;
        const VALUE: f32 = <Self as F32Ty>::__DATA;
    }
    pub struct F32Impl<const T: u32>;
    #[automatically_derived]
    impl<const T: u32> ::core::fmt::Debug for F32Impl<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "F32Impl")
        }
    }
    #[automatically_derived]
    impl<const T: u32> ::core::marker::Copy for F32Impl<T> {}
    #[automatically_derived]
    impl<const T: u32> ::core::clone::Clone for F32Impl<T> {
        #[inline]
        fn clone(&self) -> F32Impl<T> {
            *self
        }
    }
    impl<T: PrimitiveTraits<DATATYPE = f32>> F32Ty for T {
        const __DATA: f32 = <T as PrimitiveTraits>::__DATA;
    }
    impl<const T: u32> PrimitiveTraits for F32Impl<T> {
        type DATATYPE = f32;
        const __DATA: f32 = unsafe { transmute(T) };
    }
    impl<const T: u32> ConstStructPrimData for F32Impl<T> {
        type Data = f32;
        const __DATA: f32 = <F32Impl<T> as F32Ty>::__DATA;
    }
    pub trait U32Ty {
        const __DATA: u32;
        const VALUE: u32 = Self::__DATA;
    }
    pub struct U32Impl<const T: u32>;
    #[automatically_derived]
    impl<const T: u32> ::core::fmt::Debug for U32Impl<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "U32Impl")
        }
    }
    #[automatically_derived]
    impl<const T: u32> ::core::marker::Copy for U32Impl<T> {}
    #[automatically_derived]
    impl<const T: u32> ::core::clone::Clone for U32Impl<T> {
        #[inline]
        fn clone(&self) -> U32Impl<T> {
            *self
        }
    }
    impl<T: PrimitiveTraits<DATATYPE = u32>> U32Ty for T {
        const __DATA: u32 = <T as PrimitiveTraits>::__DATA;
    }
    impl<const T: u32> PrimitiveTraits for U32Impl<T> {
        type DATATYPE = u32;
        const __DATA: u32 = unsafe { transmute(T) };
    }
    impl<const T: u32> ConstStructPrimData for U32Impl<T> {
        type Data = u32;
        const __DATA: u32 = <U32Impl<T> as U32Ty>::__DATA;
    }
    pub mod some {
        pub trait PrimitiveTraits {
            type DATATYPE;
            const __DATA: Self::DATATYPE;
        }
        pub trait OptionTy<T> {
            const __DATA: Option<T>;
            const VALUE: Option<T> = Self::__DATA;
        }
        pub struct OptionImpl<T: PrimitiveTraits> {
            __phantom: core::marker::PhantomData<T>,
        }
        impl<T: PrimitiveTraits> OptionTy<T::DATATYPE> for OptionImpl<T> {
            const __DATA: Option<T::DATATYPE> = Some(<T as PrimitiveTraits>::__DATA);
        }
        impl<T: PrimitiveTraits> PrimitiveTraits for OptionImpl<T> {
            type DATATYPE = Option<T::DATATYPE>;
            const __DATA: Self::DATATYPE = Some(<T as PrimitiveTraits>::__DATA);
        }
        pub struct NoneImpl;
        impl<T> OptionTy<T> for NoneImpl {
            const __DATA: Option<T> = None;
        }
    }
    pub const fn tester_inner<T: F32Ty>() -> f32 {
        T::__DATA
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "primitive::call_tester"]
    pub const call_tester: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("primitive::call_tester"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "crates/test_trait_code/src/primitive.rs",
            start_line: 117usize,
            start_col: 8usize,
            end_line: 117usize,
            end_col: 19usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(call_tester()),
        ),
    };
    pub fn call_tester() {
        if true {
            match (
                &tester_inner::<
                    crate::primitive::F32Impl<
                        { unsafe { core::mem::transmute::<f32, u32>(-0.5) } },
                    >,
                >(),
                &-0.5,
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        if true {
            match (
                &tester_inner::<
                    crate::primitive::F32Impl<
                        { unsafe { core::mem::transmute::<f32, u32>(-25.333) } },
                    >,
                >(),
                &-25.333,
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
    }
}
mod setting {
    use crate::{
        pre::ConstStructTraits, primitive::some::PrimitiveTraits, TestSettingManual,
    };
    pub const WINDOW_SETTING_MANUAL: TestSettingManual<20> = TestSettingManual {
        test_data: Some(5),
        test_data2: None,
        test_data3: 0,
        test_data4: [0; 20],
        str: "abc_def",
    };
    pub struct WindowSettingManualTy;
    impl PrimitiveTraits for WindowSettingManualTy {
        type DATATYPE = TestSettingManual<20>;
        const __DATA: Self::DATATYPE = WINDOW_SETTING_MANUAL;
    }
}
mod struct_prim {
    use crate::{
        pre::ConstStructTraits, primitive::some::PrimitiveTraits, TestSettingManual,
    };
    pub struct ConstStructPrimAny<Key, Tail> {
        pub __phantom: core::marker::PhantomData<(Key, Tail)>,
    }
    pub trait ConstStructPrimData {
        type Data;
        const __DATA: Self::Data;
    }
    pub struct ConstStructPrimU32<const U: u32> {}
    impl<const U: u32> ConstStructPrimData for ConstStructPrimU32<U> {
        type Data = u32;
        const __DATA: Self::Data = U;
    }
    pub struct ConstStructPrimUsize<const U: usize> {}
    impl<const U: usize> ConstStructPrimData for ConstStructPrimUsize<U> {
        type Data = usize;
        const __DATA: Self::Data = U;
    }
    pub struct ConstStructPrimU8Vec<const P0: u128, const SIZE: usize, Tail> {
        pub __phantom: core::marker::PhantomData<Tail>,
    }
    pub struct ConstStructPrimU8VecLimit<const SIZE: usize, Tail> {
        pub __phantom: core::marker::PhantomData<Tail>,
    }
    impl<
        const SIZE: usize,
        const OLD_SIZE: usize,
        T: ConstStructPrimData<Data = [u8; OLD_SIZE]>,
    > ConstStructPrimData for ConstStructPrimU8VecLimit<SIZE, T> {
        type Data = [u8; SIZE];
        const __DATA: Self::Data = {
            let mut new_data = [0u8; SIZE];
            let old_data = T::__DATA;
            let mut i = 0;
            while i < SIZE {
                if i < OLD_SIZE {
                    new_data[i] = old_data[i];
                } else {
                    new_data[i] = 0;
                }
                i += 1;
            }
            new_data
        };
    }
    pub struct ConstStructPrimEnd;
    impl<
        const P0: u128,
        const SIZE: usize,
        const OLD_SIZE: usize,
        T: ConstStructPrimData<Data = [u8; OLD_SIZE]>,
    > ConstStructPrimData for ConstStructPrimU8Vec<P0, SIZE, T> {
        type Data = [u8; SIZE];
        const __DATA: Self::Data = {
            let mut new_data = [0u8; SIZE];
            let old_data = T::__DATA;
            let mut i = 0;
            while i < SIZE - 16 {
                new_data[i] = old_data[i];
                i += 1;
            }
            let u32_data: [u8; 16] = unsafe { core::mem::transmute(P0) };
            while i < SIZE {
                new_data[i] = u32_data[i + 16 - SIZE];
                i += 1;
            }
            new_data
        };
    }
    impl<const P0: u128, const SIZE: usize> ConstStructPrimData
    for ConstStructPrimU8Vec<P0, SIZE, ConstStructPrimEnd> {
        type Data = [u8; SIZE];
        const __DATA: Self::Data = {
            let mut new_data = [0u8; SIZE];
            let u32_data: [u8; 16] = unsafe { core::mem::transmute(P0) };
            let mut i = 0;
            while i < SIZE {
                new_data[i] = u32_data[i];
                i += 1;
            }
            new_data
        };
    }
    pub struct ConstStructPrimU8VecRef<const LEN: usize, Tail> {
        pub __phantom: core::marker::PhantomData<Tail>,
    }
    impl<
        const SIZE: usize,
        const LEN: usize,
        T: ConstStructPrimData<Data = [u8; SIZE]>,
    > ConstStructPrimData for ConstStructPrimU8VecRef<LEN, T> {
        type Data = &'static [u8];
        const __DATA: &'static [u8] = {
            if LEN < SIZE {
                unsafe {
                    core::slice::from_raw_parts::<'static, u8>(T::__DATA.as_ptr(), LEN)
                }
            } else {
                unsafe {
                    core::slice::from_raw_parts::<'static, u8>(T::__DATA.as_ptr(), SIZE)
                }
            }
        };
    }
    pub struct ConstStructPrimStrRef<Tail> {
        pub __phantom: core::marker::PhantomData<Tail>,
    }
    impl<T: ConstStructPrimData<Data = &'static [u8]>> ConstStructPrimData
    for ConstStructPrimStrRef<T> {
        type Data = &'static str;
        const __DATA: &'static str = reduce_from_utf8(T::__DATA);
    }
    pub type StrWrapper5<
        const A: u128,
        const B: u128,
        const C: u128,
        const D: u128,
        const E: u128,
        const LEN: usize,
    > = ConstStructPrimStrRef<
        ConstStructPrimU8VecRef<
            LEN,
            ConstStructPrimU8Vec<
                E,
                80,
                ConstStructPrimU8Vec<
                    D,
                    64,
                    ConstStructPrimU8Vec<
                        C,
                        48,
                        ConstStructPrimU8Vec<
                            B,
                            32,
                            ConstStructPrimU8Vec<A, 16, ConstStructPrimEnd>,
                        >,
                    >,
                >,
            >,
        >,
    >;
    pub const fn str_to_u128<const OFFSET: usize>(s: &str) -> u128 {
        let chars = s.as_bytes();
        let chars_len = chars.len();
        let mut target_chars = [0u8; 16];
        let mut i = 0;
        while i + OFFSET < chars_len && i < 16 {
            target_chars[i] = chars[i + OFFSET];
            i += 1;
        }
        unsafe { core::mem::transmute(target_chars) }
    }
    pub const fn vec_u8_to_u128<const OFFSET: usize>(v: &[u8]) -> u128 {
        let v_len = v.len();
        let mut target_chars = [0u8; 16];
        let mut i = 0;
        while i + OFFSET < v_len && i < 16 {
            target_chars[i] = v[i + OFFSET];
            i += 1;
        }
        unsafe { core::mem::transmute(target_chars) }
    }
    pub const fn reduce_from_utf8(v: &'static [u8]) -> &str {
        let mut i = v.len();
        while i > 0 {
            match core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(v.as_ptr(), i)
            }) {
                Ok(data) => return data,
                Err(_) => i -= 1,
            };
        }
        ""
    }
    pub struct ConstStructPrimOption<const B: bool, U> {
        pub __phantom: core::marker::PhantomData<U>,
    }
    impl<const B: bool, U: ConstStructPrimData> ConstStructPrimData
    for ConstStructPrimOption<B, U> {
        type Data = Option<U::Data>;
        const __DATA: Self::Data = if B {
            Some(<U as ConstStructPrimData>::__DATA)
        } else {
            None
        };
    }
    pub struct ConstStructPrimNone;
}
mod tester {
    use crate::{TestSettingManual, TestSettingManualTy};
    pub fn tester<T: TestSettingManualTy<20>>() {
        let t = T::TEST_DATA;
        let sl = T::__DATA;
        {
            ::std::io::_print(format_args!("{0:?}\n", t));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", sl));
        };
    }
    pub fn tester_2(test_setting_manual: TestSettingManual<20>) {
        let t = test_setting_manual.test_data;
        {
            ::std::io::_print(format_args!("{0:?}\n", t));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", test_setting_manual));
        };
    }
    pub struct Tester<T: TestSettingManualTy<20>> {
        _phantom: core::marker::PhantomData<T>,
    }
    impl<T: TestSettingManualTy<20>> Tester<T> {
        pub fn new() -> Self {
            Self {
                _phantom: core::marker::PhantomData,
            }
        }
        pub fn tester(&self) {
            let t = T::TEST_DATA;
            {
                ::std::io::_print(format_args!("{0:?}\n", t));
            };
        }
    }
    pub struct Tester2 {
        test_setting_manual: TestSettingManual<20>,
    }
    impl Tester2 {
        pub fn new(test_setting_manual: TestSettingManual<20>) -> Self {
            Self { test_setting_manual }
        }
        pub fn tester(&self) {
            let t = self.test_setting_manual.test_data;
            {
                ::std::io::_print(format_args!("{0:?}\n", t));
            };
        }
    }
}
#[allow(dead_code)]
fn main() {
    tester::<setting::WindowSettingManualTy>();
    tester_2(WINDOW_SETTING_MANUAL);
    match { #[cfg(not(feature = "dynamic"))] { Option::<&str>::None } } {
        Some(data) => {
            ::std::io::_print(format_args!("data: {0}\n", data));
        }
        None => {
            ::std::io::_print(format_args!("data: None\n"));
        }
    }
}
pub struct TestSettingManual<const T: usize> {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: [u8; T],
    str: &'static str,
}
#[automatically_derived]
impl<const T: usize> ::core::fmt::Debug for TestSettingManual<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "TestSettingManual",
            "test_data",
            &self.test_data,
            "test_data2",
            &self.test_data2,
            "test_data3",
            &self.test_data3,
            "test_data4",
            &self.test_data4,
            "str",
            &&self.str,
        )
    }
}
impl<const T: usize> TestSettingManual<T> {
    pub const fn get_const_generics_t<const T2: usize>(
        _: TestSettingManual<T2>,
    ) -> usize {
        T2
    }
}
pub trait TestSettingManualTy<
    const T: usize,
>: PrimitiveTraits<DATATYPE = TestSettingManual<T>> {
    const TEST_DATA: Option<u32> = <Self as PrimitiveTraits>::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = Self::__DATA.test_data2;
    const TEST_DATA3: u32 = Self::__DATA.test_data3;
    const TEST_DATA4: [u8; T] = Self::__DATA.test_data4;
    const STR: &'static str = Self::__DATA.str;
}
impl<
    const T: usize,
    U: PrimitiveTraits<DATATYPE = TestSettingManual<T>>,
> TestSettingManualTy<T> for U {}
type TestSettingManualTyPrimWrapper<const T: usize, A, B, C, D, S> = ConstStructPrimAny<
    TestSettingManual<T>,
    ConstStructPrimAny<
        A,
        ConstStructPrimAny<
            B,
            ConstStructPrimAny<
                C,
                ConstStructPrimAny<D, ConstStructPrimAny<S, ConstStructPrimEnd>>,
            >,
        >,
    >,
>;
impl<
    const T: usize,
    A: ConstStructPrimData<Data = Option<u32>>,
    B: ConstStructPrimData<Data = Option<Option<u32>>>,
    C: ConstStructPrimData<Data = u32>,
    D: ConstStructPrimData<Data = [u8; T]>,
    S: ConstStructPrimData<Data = &'static str>,
> PrimitiveTraits for TestSettingManualTyPrimWrapper<T, A, B, C, D, S> {
    type DATATYPE = TestSettingManual<T>;
    const __DATA: Self::DATATYPE = {
        TestSettingManual {
            test_data: <A as ConstStructPrimData>::__DATA,
            test_data2: <B as ConstStructPrimData>::__DATA,
            test_data3: <C as ConstStructPrimData>::__DATA,
            test_data4: <D as ConstStructPrimData>::__DATA,
            str: <S as ConstStructPrimData>::__DATA,
        }
    };
}
impl TestSettingManual<20> {
    pub const fn default() -> Self {
        Self {
            test_data: None,
            test_data2: None,
            test_data3: 0,
            test_data4: [
                1,
                2,
                3,
                5,
                7,
                11,
                13,
                17,
                19,
                23,
                29,
                31,
                37,
                41,
                43,
                47,
                53,
                59,
                61,
                67,
            ],
            str: "abc_def",
        }
    }
}
fn tester_with_option<const U: usize, T: OptionTy<Option<TestSettingManual<U>>>>() {
    let t = T::__DATA;
    {
        ::std::io::_print(format_args!("{0:?}\n", t));
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", T::__DATA));
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "call_tester_prim"]
pub const call_tester_prim: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("call_tester_prim"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/test_trait_code/src/main.rs",
        start_line: 215usize,
        start_col: 4usize,
        end_line: 215usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::UnitTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(call_tester_prim()),
    ),
};
fn call_tester_prim() {
    tester_prim();
}
pub fn tester_prim() {
    tester_with_option::<
        14,
        crate::primitive::some::OptionImpl<
            crate::primitive::some::OptionImpl<
                ConstStructPrimAny<
                    TestSettingManual<
                        {
                            TestSettingManual::<
                                0,
                            >::get_const_generics_t({
                                TestSettingManual {
                                    test_data: Some(5),
                                    test_data2: Some(Some(10)),
                                    test_data3: 0,
                                    test_data4: [
                                        1,
                                        2,
                                        3,
                                        5,
                                        7,
                                        11,
                                        13,
                                        17,
                                        19,
                                        23,
                                        29,
                                        31,
                                        37,
                                        41,
                                    ],
                                    str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                }
                            })
                        },
                    >,
                    ConstStructPrimAny<
                        ConstStructPrimOption<
                            {
                                let v: TestSettingManual<
                                    {
                                        TestSettingManual::<
                                            0,
                                        >::get_const_generics_t({
                                            TestSettingManual {
                                                test_data: Some(5),
                                                test_data2: Some(Some(10)),
                                                test_data3: 0,
                                                test_data4: [
                                                    1,
                                                    2,
                                                    3,
                                                    5,
                                                    7,
                                                    11,
                                                    13,
                                                    17,
                                                    19,
                                                    23,
                                                    29,
                                                    31,
                                                    37,
                                                    41,
                                                ],
                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                            }
                                        })
                                    },
                                > = {
                                    TestSettingManual {
                                        test_data: Some(5),
                                        test_data2: Some(Some(10)),
                                        test_data3: 0,
                                        test_data4: [
                                            1,
                                            2,
                                            3,
                                            5,
                                            7,
                                            11,
                                            13,
                                            17,
                                            19,
                                            23,
                                            29,
                                            31,
                                            37,
                                            41,
                                        ],
                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                    }
                                };
                                v.test_data.is_some()
                            },
                            ConstStructPrimU32<
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(Some(10)),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                    ],
                                                    str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(Some(10)),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                            ],
                                            str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                        }
                                    };
                                    match v.test_data {
                                        Some(data) => data,
                                        None => 0,
                                    }
                                },
                            >,
                        >,
                        ConstStructPrimAny<
                            ConstStructPrimOption<
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(Some(10)),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                    ],
                                                    str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(Some(10)),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                            ],
                                            str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                        }
                                    };
                                    v.test_data2.is_some()
                                },
                                ConstStructPrimOption<
                                    {
                                        match {
                                            TestSettingManual {
                                                test_data: Some(5),
                                                test_data2: Some(Some(10)),
                                                test_data3: 0,
                                                test_data4: [
                                                    1,
                                                    2,
                                                    3,
                                                    5,
                                                    7,
                                                    11,
                                                    13,
                                                    17,
                                                    19,
                                                    23,
                                                    29,
                                                    31,
                                                    37,
                                                    41,
                                                ],
                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                            }
                                        }
                                            .test_data2
                                        {
                                            Some(data) => data.is_some(),
                                            None => false,
                                        }
                                    },
                                    ConstStructPrimU32<
                                        {
                                            match {
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(Some(10)),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                    ],
                                                    str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                }
                                            }
                                                .test_data2
                                            {
                                                Some(data) => {
                                                    match data {
                                                        Some(data) => data,
                                                        None => 0,
                                                    }
                                                }
                                                None => 0,
                                            }
                                        },
                                    >,
                                >,
                            >,
                            ConstStructPrimAny<
                                ConstStructPrimU32<
                                    {
                                        let v: TestSettingManual<
                                            {
                                                TestSettingManual::<
                                                    0,
                                                >::get_const_generics_t({
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                })
                                            },
                                        > = {
                                            TestSettingManual {
                                                test_data: Some(5),
                                                test_data2: Some(Some(10)),
                                                test_data3: 0,
                                                test_data4: [
                                                    1,
                                                    2,
                                                    3,
                                                    5,
                                                    7,
                                                    11,
                                                    13,
                                                    17,
                                                    19,
                                                    23,
                                                    29,
                                                    31,
                                                    37,
                                                    41,
                                                ],
                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                            }
                                        };
                                        v.test_data3
                                    },
                                >,
                                ConstStructPrimAny<
                                    ConstStructPrimU8VecLimit<
                                        {
                                            let v: TestSettingManual<
                                                {
                                                    TestSettingManual::<
                                                        0,
                                                    >::get_const_generics_t({
                                                        TestSettingManual {
                                                            test_data: Some(5),
                                                            test_data2: Some(Some(10)),
                                                            test_data3: 0,
                                                            test_data4: [
                                                                1,
                                                                2,
                                                                3,
                                                                5,
                                                                7,
                                                                11,
                                                                13,
                                                                17,
                                                                19,
                                                                23,
                                                                29,
                                                                31,
                                                                37,
                                                                41,
                                                            ],
                                                            str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                        }
                                                    })
                                                },
                                            > = {
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(Some(10)),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                    ],
                                                    str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                }
                                            };
                                            v.test_data4.len()
                                        },
                                        ConstStructPrimU8Vec<
                                            {
                                                let v: TestSettingManual<
                                                    {
                                                        TestSettingManual::<
                                                            0,
                                                        >::get_const_generics_t({
                                                            TestSettingManual {
                                                                test_data: Some(5),
                                                                test_data2: Some(Some(10)),
                                                                test_data3: 0,
                                                                test_data4: [
                                                                    1,
                                                                    2,
                                                                    3,
                                                                    5,
                                                                    7,
                                                                    11,
                                                                    13,
                                                                    17,
                                                                    19,
                                                                    23,
                                                                    29,
                                                                    31,
                                                                    37,
                                                                    41,
                                                                ],
                                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                            }
                                                        })
                                                    },
                                                > = {
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                };
                                                crate::struct_prim::vec_u8_to_u128::<16>(&v.test_data4)
                                            },
                                            32,
                                            ConstStructPrimU8Vec<
                                                {
                                                    let v: TestSettingManual<
                                                        {
                                                            TestSettingManual::<
                                                                0,
                                                            >::get_const_generics_t({
                                                                TestSettingManual {
                                                                    test_data: Some(5),
                                                                    test_data2: Some(Some(10)),
                                                                    test_data3: 0,
                                                                    test_data4: [
                                                                        1,
                                                                        2,
                                                                        3,
                                                                        5,
                                                                        7,
                                                                        11,
                                                                        13,
                                                                        17,
                                                                        19,
                                                                        23,
                                                                        29,
                                                                        31,
                                                                        37,
                                                                        41,
                                                                    ],
                                                                    str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                                }
                                                            })
                                                        },
                                                    > = {
                                                        TestSettingManual {
                                                            test_data: Some(5),
                                                            test_data2: Some(Some(10)),
                                                            test_data3: 0,
                                                            test_data4: [
                                                                1,
                                                                2,
                                                                3,
                                                                5,
                                                                7,
                                                                11,
                                                                13,
                                                                17,
                                                                19,
                                                                23,
                                                                29,
                                                                31,
                                                                37,
                                                                41,
                                                            ],
                                                            str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                        }
                                                    };
                                                    crate::struct_prim::vec_u8_to_u128::<0>(&v.test_data4)
                                                },
                                                16,
                                                ConstStructPrimEnd,
                                            >,
                                        >,
                                    >,
                                    ConstStructPrimAny<
                                        crate::struct_prim::StrWrapper5<
                                            {
                                                let v: TestSettingManual<
                                                    {
                                                        TestSettingManual::<
                                                            0,
                                                        >::get_const_generics_t({
                                                            TestSettingManual {
                                                                test_data: Some(5),
                                                                test_data2: Some(Some(10)),
                                                                test_data3: 0,
                                                                test_data4: [
                                                                    1,
                                                                    2,
                                                                    3,
                                                                    5,
                                                                    7,
                                                                    11,
                                                                    13,
                                                                    17,
                                                                    19,
                                                                    23,
                                                                    29,
                                                                    31,
                                                                    37,
                                                                    41,
                                                                ],
                                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                            }
                                                        })
                                                    },
                                                > = {
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                };
                                                crate::struct_prim::str_to_u128::<0>(v.str)
                                            },
                                            {
                                                let v: TestSettingManual<
                                                    {
                                                        TestSettingManual::<
                                                            0,
                                                        >::get_const_generics_t({
                                                            TestSettingManual {
                                                                test_data: Some(5),
                                                                test_data2: Some(Some(10)),
                                                                test_data3: 0,
                                                                test_data4: [
                                                                    1,
                                                                    2,
                                                                    3,
                                                                    5,
                                                                    7,
                                                                    11,
                                                                    13,
                                                                    17,
                                                                    19,
                                                                    23,
                                                                    29,
                                                                    31,
                                                                    37,
                                                                    41,
                                                                ],
                                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                            }
                                                        })
                                                    },
                                                > = {
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                };
                                                crate::struct_prim::str_to_u128::<16>(v.str)
                                            },
                                            {
                                                let v: TestSettingManual<
                                                    {
                                                        TestSettingManual::<
                                                            0,
                                                        >::get_const_generics_t({
                                                            TestSettingManual {
                                                                test_data: Some(5),
                                                                test_data2: Some(Some(10)),
                                                                test_data3: 0,
                                                                test_data4: [
                                                                    1,
                                                                    2,
                                                                    3,
                                                                    5,
                                                                    7,
                                                                    11,
                                                                    13,
                                                                    17,
                                                                    19,
                                                                    23,
                                                                    29,
                                                                    31,
                                                                    37,
                                                                    41,
                                                                ],
                                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                            }
                                                        })
                                                    },
                                                > = {
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                };
                                                crate::struct_prim::str_to_u128::<32>(v.str)
                                            },
                                            {
                                                let v: TestSettingManual<
                                                    {
                                                        TestSettingManual::<
                                                            0,
                                                        >::get_const_generics_t({
                                                            TestSettingManual {
                                                                test_data: Some(5),
                                                                test_data2: Some(Some(10)),
                                                                test_data3: 0,
                                                                test_data4: [
                                                                    1,
                                                                    2,
                                                                    3,
                                                                    5,
                                                                    7,
                                                                    11,
                                                                    13,
                                                                    17,
                                                                    19,
                                                                    23,
                                                                    29,
                                                                    31,
                                                                    37,
                                                                    41,
                                                                ],
                                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                            }
                                                        })
                                                    },
                                                > = {
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                };
                                                crate::struct_prim::str_to_u128::<48>(v.str)
                                            },
                                            {
                                                let v: TestSettingManual<
                                                    {
                                                        TestSettingManual::<
                                                            0,
                                                        >::get_const_generics_t({
                                                            TestSettingManual {
                                                                test_data: Some(5),
                                                                test_data2: Some(Some(10)),
                                                                test_data3: 0,
                                                                test_data4: [
                                                                    1,
                                                                    2,
                                                                    3,
                                                                    5,
                                                                    7,
                                                                    11,
                                                                    13,
                                                                    17,
                                                                    19,
                                                                    23,
                                                                    29,
                                                                    31,
                                                                    37,
                                                                    41,
                                                                ],
                                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                            }
                                                        })
                                                    },
                                                > = {
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                };
                                                crate::struct_prim::str_to_u128::<64>(v.str)
                                            },
                                            {
                                                let v: TestSettingManual<
                                                    {
                                                        TestSettingManual::<
                                                            0,
                                                        >::get_const_generics_t({
                                                            TestSettingManual {
                                                                test_data: Some(5),
                                                                test_data2: Some(Some(10)),
                                                                test_data3: 0,
                                                                test_data4: [
                                                                    1,
                                                                    2,
                                                                    3,
                                                                    5,
                                                                    7,
                                                                    11,
                                                                    13,
                                                                    17,
                                                                    19,
                                                                    23,
                                                                    29,
                                                                    31,
                                                                    37,
                                                                    41,
                                                                ],
                                                                str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                            }
                                                        })
                                                    },
                                                > = {
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(Some(10)),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                        ],
                                                        str: "おはようございます。あなたの名前は何ですか？ 私の名前は、コンピュータです。",
                                                    }
                                                };
                                                v.str.len()
                                            },
                                        >,
                                        ConstStructPrimEnd,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >();
    let ty: ConstStructPrimAny<
        TestSettingManual<
            {
                TestSettingManual::<
                    0,
                >::get_const_generics_t({
                    TestSettingManual {
                        test_data: Some(5),
                        test_data2: Some(None),
                        test_data3: 0,
                        test_data4: [
                            1,
                            2,
                            3,
                            5,
                            7,
                            11,
                            13,
                            17,
                            19,
                            23,
                            29,
                            31,
                            37,
                            41,
                            43,
                            47,
                            53,
                            59,
                            61,
                            67,
                        ],
                        str: "abc_def",
                    }
                })
            },
        >,
        ConstStructPrimAny<
            ConstStructPrimOption<
                {
                    let v: TestSettingManual<
                        {
                            TestSettingManual::<
                                0,
                            >::get_const_generics_t({
                                TestSettingManual {
                                    test_data: Some(5),
                                    test_data2: Some(None),
                                    test_data3: 0,
                                    test_data4: [
                                        1,
                                        2,
                                        3,
                                        5,
                                        7,
                                        11,
                                        13,
                                        17,
                                        19,
                                        23,
                                        29,
                                        31,
                                        37,
                                        41,
                                        43,
                                        47,
                                        53,
                                        59,
                                        61,
                                        67,
                                    ],
                                    str: "abc_def",
                                }
                            })
                        },
                    > = {
                        TestSettingManual {
                            test_data: Some(5),
                            test_data2: Some(None),
                            test_data3: 0,
                            test_data4: [
                                1,
                                2,
                                3,
                                5,
                                7,
                                11,
                                13,
                                17,
                                19,
                                23,
                                29,
                                31,
                                37,
                                41,
                                43,
                                47,
                                53,
                                59,
                                61,
                                67,
                            ],
                            str: "abc_def",
                        }
                    };
                    v.test_data.is_some()
                },
                ConstStructPrimU32<
                    {
                        let v: TestSettingManual<
                            {
                                TestSettingManual::<
                                    0,
                                >::get_const_generics_t({
                                    TestSettingManual {
                                        test_data: Some(5),
                                        test_data2: Some(None),
                                        test_data3: 0,
                                        test_data4: [
                                            1,
                                            2,
                                            3,
                                            5,
                                            7,
                                            11,
                                            13,
                                            17,
                                            19,
                                            23,
                                            29,
                                            31,
                                            37,
                                            41,
                                            43,
                                            47,
                                            53,
                                            59,
                                            61,
                                            67,
                                        ],
                                        str: "abc_def",
                                    }
                                })
                            },
                        > = {
                            TestSettingManual {
                                test_data: Some(5),
                                test_data2: Some(None),
                                test_data3: 0,
                                test_data4: [
                                    1,
                                    2,
                                    3,
                                    5,
                                    7,
                                    11,
                                    13,
                                    17,
                                    19,
                                    23,
                                    29,
                                    31,
                                    37,
                                    41,
                                    43,
                                    47,
                                    53,
                                    59,
                                    61,
                                    67,
                                ],
                                str: "abc_def",
                            }
                        };
                        match v.test_data {
                            Some(data) => data,
                            None => 0,
                        }
                    },
                >,
            >,
            ConstStructPrimAny<
                ConstStructPrimOption<
                    {
                        let v: TestSettingManual<
                            {
                                TestSettingManual::<
                                    0,
                                >::get_const_generics_t({
                                    TestSettingManual {
                                        test_data: Some(5),
                                        test_data2: Some(None),
                                        test_data3: 0,
                                        test_data4: [
                                            1,
                                            2,
                                            3,
                                            5,
                                            7,
                                            11,
                                            13,
                                            17,
                                            19,
                                            23,
                                            29,
                                            31,
                                            37,
                                            41,
                                            43,
                                            47,
                                            53,
                                            59,
                                            61,
                                            67,
                                        ],
                                        str: "abc_def",
                                    }
                                })
                            },
                        > = {
                            TestSettingManual {
                                test_data: Some(5),
                                test_data2: Some(None),
                                test_data3: 0,
                                test_data4: [
                                    1,
                                    2,
                                    3,
                                    5,
                                    7,
                                    11,
                                    13,
                                    17,
                                    19,
                                    23,
                                    29,
                                    31,
                                    37,
                                    41,
                                    43,
                                    47,
                                    53,
                                    59,
                                    61,
                                    67,
                                ],
                                str: "abc_def",
                            }
                        };
                        v.test_data2.is_some()
                    },
                    ConstStructPrimOption<
                        {
                            match {
                                TestSettingManual {
                                    test_data: Some(5),
                                    test_data2: Some(None),
                                    test_data3: 0,
                                    test_data4: [
                                        1,
                                        2,
                                        3,
                                        5,
                                        7,
                                        11,
                                        13,
                                        17,
                                        19,
                                        23,
                                        29,
                                        31,
                                        37,
                                        41,
                                        43,
                                        47,
                                        53,
                                        59,
                                        61,
                                        67,
                                    ],
                                    str: "abc_def",
                                }
                            }
                                .test_data2
                            {
                                Some(data) => data.is_some(),
                                None => false,
                            }
                        },
                        ConstStructPrimU32<
                            {
                                match {
                                    TestSettingManual {
                                        test_data: Some(5),
                                        test_data2: Some(None),
                                        test_data3: 0,
                                        test_data4: [
                                            1,
                                            2,
                                            3,
                                            5,
                                            7,
                                            11,
                                            13,
                                            17,
                                            19,
                                            23,
                                            29,
                                            31,
                                            37,
                                            41,
                                            43,
                                            47,
                                            53,
                                            59,
                                            61,
                                            67,
                                        ],
                                        str: "abc_def",
                                    }
                                }
                                    .test_data2
                                {
                                    Some(data) => {
                                        match data {
                                            Some(data) => data,
                                            None => 0,
                                        }
                                    }
                                    None => 0,
                                }
                            },
                        >,
                    >,
                >,
                ConstStructPrimAny<
                    ConstStructPrimU32<
                        {
                            let v: TestSettingManual<
                                {
                                    TestSettingManual::<
                                        0,
                                    >::get_const_generics_t({
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    })
                                },
                            > = {
                                TestSettingManual {
                                    test_data: Some(5),
                                    test_data2: Some(None),
                                    test_data3: 0,
                                    test_data4: [
                                        1,
                                        2,
                                        3,
                                        5,
                                        7,
                                        11,
                                        13,
                                        17,
                                        19,
                                        23,
                                        29,
                                        31,
                                        37,
                                        41,
                                        43,
                                        47,
                                        53,
                                        59,
                                        61,
                                        67,
                                    ],
                                    str: "abc_def",
                                }
                            };
                            v.test_data3
                        },
                    >,
                    ConstStructPrimAny<
                        ConstStructPrimU8VecLimit<
                            {
                                let v: TestSettingManual<
                                    {
                                        TestSettingManual::<
                                            0,
                                        >::get_const_generics_t({
                                            TestSettingManual {
                                                test_data: Some(5),
                                                test_data2: Some(None),
                                                test_data3: 0,
                                                test_data4: [
                                                    1,
                                                    2,
                                                    3,
                                                    5,
                                                    7,
                                                    11,
                                                    13,
                                                    17,
                                                    19,
                                                    23,
                                                    29,
                                                    31,
                                                    37,
                                                    41,
                                                    43,
                                                    47,
                                                    53,
                                                    59,
                                                    61,
                                                    67,
                                                ],
                                                str: "abc_def",
                                            }
                                        })
                                    },
                                > = {
                                    TestSettingManual {
                                        test_data: Some(5),
                                        test_data2: Some(None),
                                        test_data3: 0,
                                        test_data4: [
                                            1,
                                            2,
                                            3,
                                            5,
                                            7,
                                            11,
                                            13,
                                            17,
                                            19,
                                            23,
                                            29,
                                            31,
                                            37,
                                            41,
                                            43,
                                            47,
                                            53,
                                            59,
                                            61,
                                            67,
                                        ],
                                        str: "abc_def",
                                    }
                                };
                                v.test_data4.len()
                            },
                            ConstStructPrimU8Vec<
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(None),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                        43,
                                                        47,
                                                        53,
                                                        59,
                                                        61,
                                                        67,
                                                    ],
                                                    str: "abc_def",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    };
                                    crate::struct_prim::vec_u8_to_u128::<16>(&v.test_data4)
                                },
                                32,
                                ConstStructPrimU8Vec<
                                    {
                                        let v: TestSettingManual<
                                            {
                                                TestSettingManual::<
                                                    0,
                                                >::get_const_generics_t({
                                                    TestSettingManual {
                                                        test_data: Some(5),
                                                        test_data2: Some(None),
                                                        test_data3: 0,
                                                        test_data4: [
                                                            1,
                                                            2,
                                                            3,
                                                            5,
                                                            7,
                                                            11,
                                                            13,
                                                            17,
                                                            19,
                                                            23,
                                                            29,
                                                            31,
                                                            37,
                                                            41,
                                                            43,
                                                            47,
                                                            53,
                                                            59,
                                                            61,
                                                            67,
                                                        ],
                                                        str: "abc_def",
                                                    }
                                                })
                                            },
                                        > = {
                                            TestSettingManual {
                                                test_data: Some(5),
                                                test_data2: Some(None),
                                                test_data3: 0,
                                                test_data4: [
                                                    1,
                                                    2,
                                                    3,
                                                    5,
                                                    7,
                                                    11,
                                                    13,
                                                    17,
                                                    19,
                                                    23,
                                                    29,
                                                    31,
                                                    37,
                                                    41,
                                                    43,
                                                    47,
                                                    53,
                                                    59,
                                                    61,
                                                    67,
                                                ],
                                                str: "abc_def",
                                            }
                                        };
                                        crate::struct_prim::vec_u8_to_u128::<0>(&v.test_data4)
                                    },
                                    16,
                                    ConstStructPrimEnd,
                                >,
                            >,
                        >,
                        ConstStructPrimAny<
                            crate::struct_prim::StrWrapper5<
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(None),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                        43,
                                                        47,
                                                        53,
                                                        59,
                                                        61,
                                                        67,
                                                    ],
                                                    str: "abc_def",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    };
                                    crate::struct_prim::str_to_u128::<0>(v.str)
                                },
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(None),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                        43,
                                                        47,
                                                        53,
                                                        59,
                                                        61,
                                                        67,
                                                    ],
                                                    str: "abc_def",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    };
                                    crate::struct_prim::str_to_u128::<16>(v.str)
                                },
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(None),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                        43,
                                                        47,
                                                        53,
                                                        59,
                                                        61,
                                                        67,
                                                    ],
                                                    str: "abc_def",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    };
                                    crate::struct_prim::str_to_u128::<32>(v.str)
                                },
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(None),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                        43,
                                                        47,
                                                        53,
                                                        59,
                                                        61,
                                                        67,
                                                    ],
                                                    str: "abc_def",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    };
                                    crate::struct_prim::str_to_u128::<48>(v.str)
                                },
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(None),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                        43,
                                                        47,
                                                        53,
                                                        59,
                                                        61,
                                                        67,
                                                    ],
                                                    str: "abc_def",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    };
                                    crate::struct_prim::str_to_u128::<64>(v.str)
                                },
                                {
                                    let v: TestSettingManual<
                                        {
                                            TestSettingManual::<
                                                0,
                                            >::get_const_generics_t({
                                                TestSettingManual {
                                                    test_data: Some(5),
                                                    test_data2: Some(None),
                                                    test_data3: 0,
                                                    test_data4: [
                                                        1,
                                                        2,
                                                        3,
                                                        5,
                                                        7,
                                                        11,
                                                        13,
                                                        17,
                                                        19,
                                                        23,
                                                        29,
                                                        31,
                                                        37,
                                                        41,
                                                        43,
                                                        47,
                                                        53,
                                                        59,
                                                        61,
                                                        67,
                                                    ],
                                                    str: "abc_def",
                                                }
                                            })
                                        },
                                    > = {
                                        TestSettingManual {
                                            test_data: Some(5),
                                            test_data2: Some(None),
                                            test_data3: 0,
                                            test_data4: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                7,
                                                11,
                                                13,
                                                17,
                                                19,
                                                23,
                                                29,
                                                31,
                                                37,
                                                41,
                                                43,
                                                47,
                                                53,
                                                59,
                                                61,
                                                67,
                                            ],
                                            str: "abc_def",
                                        }
                                    };
                                    v.str.len()
                                },
                            >,
                            ConstStructPrimEnd,
                        >,
                    >,
                >,
            >,
        >,
    > = ConstStructPrimAny {
        __phantom: core::marker::PhantomData,
    };
    {
        ::std::io::_print(format_args!("size: {0:?}\n", core::mem::size_of_val(&ty)));
    };
}
const PI: f32 = 3.14159;
struct PiTy;
impl PrimitiveTraits for PiTy {
    type DATATYPE = f32;
    const __DATA: <Self as PrimitiveTraits>::DATATYPE = PI;
}
fn tester_pi<T: F32Ty>() {
    {
        ::std::io::_print(format_args!("PI: {0}\n", T::__DATA));
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "call_tester_pi"]
pub const call_tester_pi: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("call_tester_pi"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "crates/test_trait_code/src/main.rs",
        start_line: 264usize,
        start_col: 4usize,
        end_line: 264usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::UnitTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(call_tester_pi()),
    ),
};
fn call_tester_pi() {
    tester_pi::<PiTy>();
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &call_tester_pi,
            &call_tester_prim,
            &call_macro,
            &call_macro,
            &test_test_generics,
            &call_tester,
        ],
    )
}
