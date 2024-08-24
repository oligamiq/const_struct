pub mod primitive_ty;
pub use primitive_ty::*;
pub mod option;
pub use option::*;
pub mod tuple;
pub use tuple::*;
pub mod unit;
pub use unit::*;
pub mod enum_ty;
pub use enum_ty::*;
pub mod struct_ty;
pub use struct_ty::*;
pub mod array;

pub trait PrimitiveTraits {
    type DATATYPE;
    const __DATA: Self::DATATYPE;
}
