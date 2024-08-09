pub trait ConstStruct<T> {
    const __DATA: T;
}

pub trait ConstStructPrimImplType {
    type PrimType<const U: u128, T>;
}
