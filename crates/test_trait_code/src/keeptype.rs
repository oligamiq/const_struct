pub trait KeepTypeConst<const N: usize> {
    type DATATYPE;
    const N: Self::DATATYPE;
}

pub trait KeepType<const N: usize> {
    type Type;
}

pub struct Marker<const N: usize>;
