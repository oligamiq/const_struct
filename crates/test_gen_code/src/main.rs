// pub fn tester<const A: usize, const B: usize>() {
//     println!("{:?}", A);
//     println!("{:?}", B);
// }

// pub fn main() {
//     tester::<20, 30>();
// }

use const_struct::{call_with_generics, const_struct, primitive::TupleTy, ConstStruct};

#[derive(ConstStruct)]
pub struct TestSetting<const N: usize>;

impl<const N: usize> core::fmt::Debug for TestSetting<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TestSetting")
            .field("N", &N)
            .finish()
    }
}

#[derive(ConstStruct)]
pub struct WestSetting<const N: usize>;

impl<const N: usize> core::fmt::Debug for WestSetting<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WestSetting")
            .field("N", &N)
            .finish()
    }
}

pub fn tester<const N: usize, const N2: usize, A: TupleTy<(TestSetting<N>, WestSetting<N2>)>, const N3: usize, B: TupleTy<(f32, TestSetting<N3>)>>() {
    println!("a: {:?}", A::__DATA);
    println!("b: {:?}", B::__DATA);
}

#[const_struct]
const B: TestSetting<0> = TestSetting;

#[test]
fn main() {
    call_with_generics!(tester::<(TestSetting!(BTy), WestSetting!(WestSetting::<2>)), (F32!(0.5), TestSetting!(8, TestSetting))>());
}
