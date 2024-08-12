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

#[cfg(test)]
mod tests {
    use crate::primitive::{F32Ty, U32Ty, _F32 as F32, _U32 as U32};
    use core::mem;

    use super::TupleTy;

    pub const fn tester_inner<T: TupleTy<(f32, f64)>>() -> (f32, f64) {
        T::VALUE
    }

    #[test]
    fn cal_tester() {
    }
}
