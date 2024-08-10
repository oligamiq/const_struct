use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use crate::{pre::ConstStructTraits, primitive::some::PrimitiveTraits, TestSettingManual};

pub struct ConstStructPrimAny<Key, Tail> {
    pub __phantom: core::marker::PhantomData<(Key, Tail)>,
}

pub trait ConstStructPrimData {
    type Data;
    const __DATA: Self::Data;
}

pub struct ConstStructPrimU32<const U: u32> {}

impl<const U: u32> ConstStructPrimData for ConstStructPrimU32<U> {
    type Data = u32;
    const __DATA: Self::Data = U;
}

pub struct ConstStructPrimUsize<const U: usize> {}

impl<const U: usize> ConstStructPrimData for ConstStructPrimUsize<U> {
    type Data = usize;
    const __DATA: Self::Data = U;
}

pub struct ConstStructPrimStr<const P0: u128, const SIZE: usize, Tail> {
    pub __phantom: core::marker::PhantomData<Tail>,
}

impl<
        const P0: u128,
        const SIZE: usize,
        const OLD_SIZE: usize,
        T: ConstStructPrimData<Data = [u32; OLD_SIZE]>,
    > ConstStructPrimData for ConstStructPrimStr<P0, SIZE, T>
{
    type Data = [u32; SIZE];

    // DATA + P0
    const __DATA: Self::Data = {
        let mut new_data = [0u32; SIZE];
        let old_data = T::__DATA;
        let mut i = 0;
        while i < SIZE - 4 {
            new_data[i] = old_data[i];
            i += 1;
        }
        let u32_data: [u32; 4] = unsafe { core::mem::transmute(P0) };
        new_data[SIZE - 4] = u32_data[0];
        new_data[SIZE - 3] = u32_data[1];
        new_data[SIZE - 2] = u32_data[2];
        new_data[SIZE - 1] = u32_data[3];
        new_data
    };
}

pub struct ConstStructPrimStrRef<S: ConstStructPrimRef> {
    pub __phantom: core::marker::PhantomData<S>,
}

pub struct ConstStructPrimOption<const B: bool, U> {
    pub __phantom: core::marker::PhantomData<U>,
}

impl<const B: bool, U: ConstStructPrimData> ConstStructPrimData for ConstStructPrimOption<B, U> {
    type Data = Option<U::Data>;
    const __DATA: Self::Data = if B {
        Some(<U as ConstStructPrimData>::__DATA)
    } else {
        None
    };
}

pub struct ConstStructPrimNone;

pub struct ConstStructPrimEnd;

pub trait ConstStructPrimRef {
    type Data: ?Sized;
    const __DATA: Self::Data;
}

pub struct ConstStructPrimChar<const C: char> {}

pub struct StrWrapperStruct;

pub type StrWrapper<
    const A: char,
    const B: char,
    const C: char,
    const D: char,
    const E: char,
    const F: char,
> = ConstStructPrimAny<
    ConstStructPrimChar<A>,
    ConstStructPrimAny<
        ConstStructPrimChar<B>,
        ConstStructPrimAny<
            ConstStructPrimChar<C>,
            ConstStructPrimAny<
                ConstStructPrimChar<D>,
                ConstStructPrimAny<
                    ConstStructPrimChar<E>,
                    ConstStructPrimAny<ConstStructPrimChar<F>, ConstStructPrimEnd>,
                >,
            >,
        >,
    >,
>;

impl<const A: char, const B: char, const C: char, const D: char, const E: char, const F: char>
    ConstStructPrimRef for StrWrapper<A, B, C, D, E, F>
{
    type Data = [u8; 6];
    const __DATA: [u8; 6] = {
        let data = [A as u8, B as u8, C as u8, D as u8, E as u8, F as u8];
        data
    };
}
