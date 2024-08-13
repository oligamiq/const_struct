#![no_std]
//! # const_struct
//!
//! This crate provides a way to create a struct with const generics and const values.
//! ```
//! use const_struct::{primitive::F32Ty, F32};
//! pub fn tester<A: F32Ty>() {
//!     println!("a: {:?}", A::__DATA);
//! }
//! fn main() {
//!     tester::<F32!(0.5)>();
//! }
//! ```

pub mod prelude;
pub mod primitive;
pub mod struct_prim;
pub mod util_macro;
pub use const_struct_derive::*;

pub trait ConstStructTraits<T> {
    const __DATA: T;
}
