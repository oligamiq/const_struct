use std::marker::PhantomData;

use const_struct_derive::ConstStruct;
use pre::{DefaultNone, TailAddition, TailSome, TestSettingManualTyImpl, TestSettingManualTyImplData};

mod setting;
mod pre;

fn main() {
}

#[derive(ConstStruct)]
struct TestSetting {
    a: Option<u32>,
}

impl TestSetting {
    pub const fn default() -> Self {
        Self { a: None }
    }
}


struct TestSettingManual {
    test_data: Option<u32>,
    test_data2: Option<u32>,
    test_data3: Option<u32>,
    test_data4: Option<u32>,
    test_data5: Option<u32>,
    test_data6: Option<u32>,
}

impl TestSettingManualTyImplData for TestSettingManual {
    type __A = Option<u32>;
    type __B = DefaultNone;
    type __C = DefaultNone;
    type __D = DefaultNone;
    type __TAIL = TailSome<TestSettingManualTailAdd1>;
}

// trait TestSettingManualTy
// where
//     Self:
//         TestSettingManualTyImpl<
//             <TestSettingManual as TestSettingManualTyImplData>::__A,
//             <TestSettingManual as TestSettingManualTyImplData>::__B,
//             <TestSettingManual as TestSettingManualTyImplData>::__C,
//             <TestSettingManual as TestSettingManualTyImplData>::__D,
//             <TestSettingManual as TestSettingManualTyImplData>::__TAIL
//         >,
//     <TestSettingManual as TestSettingManualTyImplData>::__TAIL: TestSettingManualTyImplData,
//     <TestSettingManual as TestSettingManualTyImplData>::__TAIL: TestSettingManualTyImpl<
//         <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__A,
//         <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__B,
//         <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__C,
//         <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__D,
//         <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__TAIL,
//     >,
// {
//     const TEST_DATA: <TestSettingManual as TestSettingManualTyImplData>::__A = TestSettingManualTyImpl::__A;
//     const TEST_DATA2: <TestSettingManual as TestSettingManualTyImplData>::__B = TestSettingManualTyImpl::__B;
//     const TEST_DATA3: <TestSettingManual as TestSettingManualTyImplData>::__C = TestSettingManualTyImpl::__C;
//     const TEST_DATA4: <TestSettingManual as TestSettingManualTyImplData>::__D = TestSettingManualTyImpl::__D;
//     const TEST_DATA9: <<TestSettingManual as TestSettingManualTyImplData>::__TAIL as TestSettingManualTyImplData>::__A = {
//         match TestSettingManualTyImpl::__TAIL {
//             TailAddition::None => TailAddition::None,
//             TailAddition::Some(v) => <<TestSettingManual as TestSettingManualTyImplData>::__TAIL as TestSettingManualTyImpl<
//                 <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__A,
//                 <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__B,
//                 <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__C,
//                 <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__D,
//                 <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__TAIL,
//             >>::__A,
//         }
//     };
//     const TEST_DATA10: <<TestSettingManual as TestSettingManualTyImplData>::__TAIL as TestSettingManualTyImplData>::__B = {
//         match TestSettingManualTyImpl::__TAIL {
//             TailAddition::None => TailAddition::None,
//             TailAddition::Some(v) => TailAddition::Some(TestSettingManualTailAdd1),
//         }
//     };
// }

trait TestSettingManualTy
where
    Self:
        TestSettingManualTyImpl<
            Option<u32>,
            DefaultNone,
            DefaultNone,
            DefaultNone,
            TailSome<TestSettingManualTailAdd1>
        >,
    TailSome<TestSettingManualTailAdd1>: TestSettingManualTyImplData,
{
    const TEST_DATA: <TestSettingManual as TestSettingManualTyImplData>::__A = TestSettingManualTyImpl::__A;
    const TEST_DATA2: <TestSettingManual as TestSettingManualTyImplData>::__B = TestSettingManualTyImpl::__B;
    const TEST_DATA3: <TestSettingManual as TestSettingManualTyImplData>::__C = TestSettingManualTyImpl::__C;
    const TEST_DATA4: <TestSettingManual as TestSettingManualTyImplData>::__D = TestSettingManualTyImpl::__D;
    const TEST_DATA9: <<TestSettingManual as TestSettingManualTyImplData>::__TAIL as TestSettingManualTyImplData>::__A = {
        match TestSettingManualTyImpl::__TAIL {
            TailAddition::None => TailAddition::None,
            TailAddition::Some(v) => <<TestSettingManual as TestSettingManualTyImplData>::__TAIL as TestSettingManualTyImpl<
                <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__A,
                <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__B,
                <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__C,
                <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__D,
                <TestSettingManualTailAdd1 as TestSettingManualTyImplData>::__TAIL,
            >>::__A,
        }
    };
    const TEST_DATA10: <<TestSettingManual as TestSettingManualTyImplData>::__TAIL as TestSettingManualTyImplData>::__B = {
        match TestSettingManualTyImpl::__TAIL {
            TailAddition::None => TailAddition::None,
            TailAddition::Some(v) => TailAddition::Some(TestSettingManualTailAdd1),
        }
    };
}


struct TestSettingManualTailAdd1;

impl TestSettingManualTyImplData for TestSettingManualTailAdd1 {
    type __A = Option<u32>;
    type __B = DefaultNone;
    type __C = DefaultNone;
    type __D = DefaultNone;
    type __TAIL = TailSome<DefaultNone>;
}

trait TestSettingManualTy
where
    Self:
        
