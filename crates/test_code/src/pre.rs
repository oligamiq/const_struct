pub trait ConstStructData<A, B, C, D, E> {
    const __A: A;
    const __B: B;
    const __C: C;
    const __D: D;
    const __E: E;
}

pub trait ConstStructTyWrapper<T: ConstStructTy> {
    const __A: <T as ConstStructTy>::__A;
    const __B: <T as ConstStructTy>::__B;
    const __C: <T as ConstStructTy>::__C;
    const __D: <T as ConstStructTy>::__D;
    const __E: <T as ConstStructTy>::__E;
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
