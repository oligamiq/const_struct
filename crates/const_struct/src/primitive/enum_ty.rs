pub struct EnumQueuePlaneHead<Type, Tail, const N: usize> {
    pub __phantom: core::marker::PhantomData<(Type, Tail)>,
}

pub struct EnumQueuePlaneDataType<Data, Tail> {
    pub __phantom: core::marker::PhantomData<(Data, Tail)>,
}

pub struct EnumQueuePlaneEnd;
