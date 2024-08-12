use super::PrimitiveTraits;

pub trait TupleTy<T> {
    const __DATA: T;
    const VALUE: T = Self::__DATA;
}

impl TupleTy<()> for () {
    const __DATA: () = ();
}

macro_rules! TupleTyByNum {
    ($($generics:ident),*) => {
        impl<$($generics: PrimitiveTraits),*, > TupleTy<($($generics::DATATYPE),*, )> for ($($generics),*, ) {
            const __DATA: ($($generics::DATATYPE),*, ) = ($($generics::__DATA),*, );
        }

        impl<$($generics: PrimitiveTraits),*, > PrimitiveTraits for ($($generics),*, ) {
            type DATATYPE = ($($generics::DATATYPE),*, );
            const __DATA: Self::DATATYPE = ($($generics::__DATA),*, );
        }
    };
}

TupleTyByNum!(A);
TupleTyByNum!(A, B);
TupleTyByNum!(A, B, C);
TupleTyByNum!(A, B, C, D);
TupleTyByNum!(A, B, C, D, E);
TupleTyByNum!(A, B, C, D, E, F);
TupleTyByNum!(A, B, C, D, E, F, G);
TupleTyByNum!(A, B, C, D, E, F, G, H);
TupleTyByNum!(A, B, C, D, E, F, G, H, I);
TupleTyByNum!(A, B, C, D, E, F, G, H, I, J);

#[cfg(test)]
mod tests {
    use crate::primitive::{_F32 as F32, _F64 as F64, _U32 as U32};
    use core::mem;

    use super::TupleTy;

    pub const fn tester_inner0<T: TupleTy<()>>() -> () {
        T::VALUE
    }

    pub const fn tester_inner1<T: TupleTy<(f32,)>>() -> (f32,) {
        T::VALUE
    }

    pub const fn tester_inner2<T: TupleTy<(f32, f64)>>() -> (f32, f64) {
        T::VALUE
    }

    pub const fn tester_inner3<T: TupleTy<(f32, (f64, u32))>>() -> (f32, (f64, u32)) {
        T::VALUE
    }

    #[test]
    fn call_tester() {
        let s: (F32!(0.4), F64!(0.3)) = unsafe { mem::zeroed() };
        assert_eq!(core::mem::size_of_val(&s), 0);
        assert_eq!(tester_inner0::<()>(), ());
        assert_eq!(tester_inner1::<(F32!(0.4), )>(), (0.4, ));
        assert_eq!(tester_inner2::<(F32!(0.4), F64!(0.3))>(), (0.4, 0.3));
        assert_eq!(tester_inner3::<(F32!(0.4), (F64!(0.3), U32!(5)))>(), (0.4, (0.3, 5)));
    }
}
