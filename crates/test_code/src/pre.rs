pub trait ConstStructImplData<A, B, C, D, E> {
    const __A: A;
    const __B: B;
    const __C: C;
    const __D: D;
    const __E: E;
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
