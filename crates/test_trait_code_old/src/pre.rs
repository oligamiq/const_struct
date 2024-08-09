pub trait ConstStructData<A, B, C, D, E> {
    const __A: A;
    const __B: B;
    const __C: C;
    const __D: D;
    const __E: E;
}

pub trait ConstStructTy {
    type __A;
    type __B;
    type __C;
    type __D;
    type __E;
}

pub struct ConstStruct<A, B, C, D, E> {
    pub __a: A,
    pub __b: B,
    pub __c: C,
    pub __d: D,
    pub __e: E,
}

pub struct Non;
