use const_struct_derive::call_with_generics;

pub fn tester<const A: usize, const B: usize>() {
    println!("{:?}", A);
    println!("{:?}", B);
}

pub fn main() {
    tester::<20, 30>();
}
