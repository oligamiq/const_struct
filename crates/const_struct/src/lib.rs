#![no_std]

pub mod prelude;
pub use const_struct_derive::*;

pub trait ConstStructTraits<T> {
    const __DATA: T;
}
