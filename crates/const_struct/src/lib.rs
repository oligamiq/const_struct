#![no_std]

pub mod prelude;
pub mod primitive;
pub mod struct_prim;
pub mod util_macro;
pub use const_struct_derive::*;

pub trait ConstStructTraits<T> {
    const __DATA: T;
}
