use super::ConstStructPrimEnd;

pub trait ConstStructPrimQueue {
    type Data;
    const __DATA: Self::Data;
}

pub struct ConstStructPrimU8Vec<const P0: u128, const SIZE: usize, Tail> {
    pub __phantom: core::marker::PhantomData<Tail>,
}

impl<const P0: u128, const SIZE: usize> ConstStructPrimQueue
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

pub struct ConstStructPrimU8VecLimit<const SIZE: usize, Tail> {
    pub __phantom: core::marker::PhantomData<Tail>,
}

impl<const SIZE: usize, const OLD_SIZE: usize, T: ConstStructPrimQueue<Data = [u8; OLD_SIZE]>>
    ConstStructPrimQueue for ConstStructPrimU8VecLimit<SIZE, T>
{
    type Data = [u8; SIZE];
    const __DATA: Self::Data = {
        let mut new_data = [0u8; SIZE];
        let old_data = T::__DATA;
        let mut i = 0;
        while i < SIZE {
            if i < OLD_SIZE {
                new_data[i] = old_data[i];
            } else {
                new_data[i] = 0;
            }
            i += 1;
        }
        new_data
    };
}

pub struct ConstStructPrimU8VecRef<const LEN: usize, Tail> {
    pub __phantom: core::marker::PhantomData<Tail>,
}

impl<const SIZE: usize, const LEN: usize, T: ConstStructPrimQueue<Data = [u8; SIZE]>>
    ConstStructPrimQueue for ConstStructPrimU8VecRef<LEN, T>
{
    type Data = &'static [u8];
    const __DATA: &'static [u8] = {
        if LEN < SIZE {
            unsafe { core::slice::from_raw_parts::<'static, u8>(T::__DATA.as_ptr(), LEN) }
        } else {
            unsafe { core::slice::from_raw_parts::<'static, u8>(T::__DATA.as_ptr(), SIZE) }
        }
    };
}
