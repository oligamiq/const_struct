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
//!
//! This crate is no_std.
//! used unsafe code:
//! * `core::mem::zeroed`
//! * `core::mem::transmute`
//!
//! See the Github README for more information.
#![no_std]

pub mod prelude;
pub mod primitive;
pub use primitive::PrimitiveTraits;
pub mod struct_prim;
pub mod util_macro;
pub use const_struct_derive::*;
pub mod keeptype;
