use super::PrimitiveTraits;

pub trait UnitTy {
    const __DATA: ();
    const VALUE: () = Self::__DATA;
}

impl PrimitiveTraits for () {
    type DATATYPE = ();
    const __DATA: Self::DATATYPE = ();
}

impl UnitTy for () {
    const __DATA: () = ();
}
