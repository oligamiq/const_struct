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

#[cfg(test)]
mod tests {
    use super::UnitTy;
    use core::mem;

    pub const fn tester_inner<T: UnitTy>() {
        T::VALUE
    }

    #[test]
    fn call_tester() {
        let s: () = unsafe { mem::zeroed() };
        assert_eq!(core::mem::size_of_val(&s), 0);
        assert_eq!(tester_inner::<()>(), ());
    }
}
