pub trait KeepType {
    type Type;
}

pub struct KeepTypeStruct<T, const N: usize> {
    __phantom: core::marker::PhantomData<T>,
}

pub struct KeepTypeConst<const N: usize>;

impl<const N: usize> KeepTypeConst<N> {
    pub const __DATA: usize = N;
}
