pub trait ConstStruct<T> {
    const __DATA: T;
}

pub trait ConstStructPrimImplType {
    type PrimType<const U: u32, T>;
}

pub struct TestSettingManualBox<const U: u32, T> {
    __phantom: core::marker::PhantomData<T>,
}

pub struct TestSettingManualEnd<const U: u32>;

impl<const S: u32> ConstStructPrimImplType for TestSettingManualEnd<S> {
    type PrimType<const U: u32, T> = TestSettingManualEnd<U>;
}
