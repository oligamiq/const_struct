pub trait ConstStruct<T> {
    const __DATA: T;
}

pub trait ConstStructPrimImplType {
    type PrimType<const U: u32, T>;
}

pub struct ConstStructPrimAny<U, T> {
    __phantom: core::marker::PhantomData<(U, T)>,
}

pub struct ConstStructPrimBoxMem32<const U: u32, T> {
    __phantom: core::marker::PhantomData<T>,
}

pub struct ConstStructPrimEnd<const U: u32>;
