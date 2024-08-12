use const_struct_derive::call_with_generics;

pub fn tester<const A: usize, const B: usize>() {
    println!("{:?}", A);
    println!("{:?}", B);
}

pub fn main() {
    tester::<20, 30>();
    // let s = tester::<expand_generics_inner!({20}, {30})>;
    // println!("{:?}", s);
    // tester::<expand_generics_inner!({20}, {30})>;

    call_with_generics!(call_tester::<
        _,
        f32,
        TestGenerics!(_, f32, TestGenerics { s: 0.6, t: [0; 56] }),
    >());
}
