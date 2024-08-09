#![no_std]

pub mod prelude;
pub mod primitive;
pub mod struct_prim;
pub use const_struct_derive::*;

pub trait ConstStructTraits<T> {
    const __DATA: T;
}
