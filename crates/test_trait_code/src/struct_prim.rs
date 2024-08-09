use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

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

pub struct ConstStructPrimStr<const P0: char, const Size: usize, Tail> {
    pub __phantom: core::marker::PhantomData<Tail>,
}

impl<const P0: char, const Size: usize, const OldSize: usize, T: ConstStructPrimData<Data = [char; OldSize]>> ConstStructPrimData for ConstStructPrimStr<P0, Size, T> {
    type Data = [char; Size];
    // DATA + P0
    const __DATA: Self::Data = {
        let mut new_data = [P0; Size];
        let new_data_mut: &mut [char; OldSize] = match (&mut new_data).first_chunk_mut() {
            Some(new_data_mut) => new_data_mut,
            None => panic!("Failed to get first chunk mutable"),
        };
        core::mem::replace(new_data_mut, T::__DATA);
        let old_data = T::__DATA;
        new_data_mut.copy_from_slice(&old_data);
        new_data
    };
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
