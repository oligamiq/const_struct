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

pub struct ConstStructPrimU8Vec<const P0: u128, const SIZE: usize, Tail> {
    pub __phantom: core::marker::PhantomData<Tail>,
}

pub struct ConstStructPrimEnd;

impl<
        const P0: u128,
        const SIZE: usize,
        const OLD_SIZE: usize,
        T: ConstStructPrimData<Data = [u8; OLD_SIZE]>,
    > ConstStructPrimData for ConstStructPrimU8Vec<P0, SIZE, T>
{
    type Data = [u8; SIZE];

    // DATA + P0
    const __DATA: Self::Data = {
        let mut new_data = [0u8; SIZE];
        let old_data = T::__DATA;
        let mut i = 0;
        while i < SIZE - 16 {
            new_data[i] = old_data[i];
            i += 1;
        }
        let u32_data: [u8; 16] = unsafe { core::mem::transmute(P0) };
        while i < SIZE {
            new_data[i] = u32_data[i + 16 - SIZE];
            i += 1;
        }
        new_data
    };
}

impl<const P0: u128, const SIZE: usize> ConstStructPrimData
    for ConstStructPrimU8Vec<P0, SIZE, ConstStructPrimEnd>
{
    type Data = [u8; SIZE];
    const __DATA: Self::Data = {
        let mut new_data = [0u8; SIZE];
        let u32_data: [u8; 16] = unsafe { core::mem::transmute(P0) };
        let mut i = 0;
        while i < SIZE {
            new_data[i] = u32_data[i];
            i += 1;
        }
        new_data
    };
}

pub struct ConstStructPrimU8VecRef<const LEN: usize, Tail> {
    pub __phantom: core::marker::PhantomData<Tail>,
}

impl<const SIZE: usize, const LEN: usize, T: ConstStructPrimData<Data = [u8; SIZE]>,
> ConstStructPrimData for ConstStructPrimU8VecRef<LEN, T> {
    type Data = &'static [u8];
    const __DATA: &'static [u8] = {
        if LEN < SIZE {
            unsafe { core::slice::from_raw_parts::<'static, u8>(T::__DATA.as_ptr(), LEN) }
        } else {
            unsafe { core::slice::from_raw_parts::<'static, u8>(T::__DATA.as_ptr(), SIZE) }
        }
    };
}

pub type StrWrapper2<const A: u128, const B: u128, const C: u128, const D: u128, const E: u128, const LEN: usize> =
ConstStructPrimU8VecRef<LEN, ConstStructPrimU8Vec<E, 80, ConstStructPrimU8Vec<D, 64, ConstStructPrimU8Vec<C, 48, ConstStructPrimU8Vec<B, 32, ConstStructPrimU8Vec<A, 16, ConstStructPrimEnd>>>>>>;

pub const fn str_to_u128<const Offset: usize>(s: &str) -> u128 {
    let chars = s.as_bytes();
    let chars_len = chars.len();
    let mut target_chars = [0u8; 16];
    let mut i = 0;
    while i + Offset < chars_len && i < 16 {
        target_chars[i] = chars[i + Offset];
        i += 1;
    }
    unsafe { core::mem::transmute(target_chars) }
}

pub const fn reduce_from_utf8(v: &'static [u8]) -> &str {
    let mut i = v.len();
    while i > 0{
        match core::str::from_utf8(unsafe { core::slice::from_raw_parts(v.as_ptr(), i) }) {
            Ok(data) => return data,
            Err(_) => i -= 1,
        };
    }
    ""
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
