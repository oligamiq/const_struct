pub trait ConstStructImplData: ConstStructImplTy {
    const __A: Self::__A;
    const __B: Self::__B;
    const __C: Self::__C;
    const __D: Self::__D;
    const __E: Self::__E;
}

pub trait ConstStructImplTy {
    type __A;
    type __B;
    type __C;
    type __D;
    type __E;
}

pub struct ConstStructImpl<A, B, C, D, E> {
    pub __a: A,
    pub __b: B,
    pub __c: C,
    pub __d: D,
    pub __e: E,
}

pub struct DefaultNone;
