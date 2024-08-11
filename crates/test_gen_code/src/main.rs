use const_struct_derive::expand_generics_inner;

pub fn tester<const A: usize, const B: usize>() {
    println!("{:?}", A);
    println!("{:?}", B);
}

pub fn main() {
    tester::<20, 30>();
    // let s = tester::<expand_generics_inner!({20}, {30})>;
    // println!("{:?}", s);
    // tester::<expand_generics_inner!({20}, {30})>;
}
