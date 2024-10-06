use const_struct::{const_struct, ConstStruct};

#[const_struct(macro_export)]
#[derive(ConstStruct, Debug)]
pub struct ConstArgs<const N: usize> {
    args: [&'static str; N],
}

pub const fn argv_buf_size<const N: usize, T: ConstArgsTy<N>>() -> u32 {
    let mut size = 0;
    let u = T::__DATA;
    let args = T::ARGS;
    // while size < N {

    // }
    // size

    0
}
