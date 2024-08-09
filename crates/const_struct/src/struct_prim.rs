use paste::paste;

pub struct ConstStructPrimAny<U, T> {
    pub __phantom: core::marker::PhantomData<(U, T)>,
}

macro_rules! ConstStructPrimBoxBySizes {
    ($base:tt) => {
        paste! {
            ConstStructPrimBoxBySizes!([<$base:camel>], $base);
        }
    };
    ($name:tt, $base:tt) => {
        paste! {
            pub struct [<ConstStructPrimBoxBySizes $name>]<const U: [<u $base>], T> {
                __phantom: core::marker::PhantomData<T>,
            }
        }
    };
}

ConstStructPrimBoxBySizes!(8);
ConstStructPrimBoxBySizes!(16);
ConstStructPrimBoxBySizes!(32);
ConstStructPrimBoxBySizes!(64);
ConstStructPrimBoxBySizes!(128);
ConstStructPrimBoxBySizes!(size);
ConstStructPrimBoxBySizes!(char, 32);

pub struct ConstStructPrimEnd;
