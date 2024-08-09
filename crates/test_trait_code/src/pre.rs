pub trait ConstStruct<T> {
    const __DATA: T;
}

pub struct ConstStructPrimAny<U, T> {
    pub __phantom: core::marker::PhantomData<(U, T)>,
}

pub struct ConstStructPrimBoxMem32<const U: u32, T> {
    __phantom: core::marker::PhantomData<T>,
}

pub struct ConstStructPrimEnd;
