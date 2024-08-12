use super::PrimitiveTraits;

pub trait TupleTy<T> {
    const __DATA: T;
    const VALUE: T = Self::__DATA;
}

impl<A: PrimitiveTraits, B: PrimitiveTraits> TupleTy<(A::DATATYPE, B::DATATYPE)> for (A, B) {
    const __DATA: (A::DATATYPE, B::DATATYPE) = (A::__DATA, B::__DATA);
}

impl<A: PrimitiveTraits, B: PrimitiveTraits> PrimitiveTraits for (A, B) {
    type DATATYPE = (A::DATATYPE, B::DATATYPE);
    const __DATA: Self::DATATYPE = (A::__DATA, B::__DATA);
}
