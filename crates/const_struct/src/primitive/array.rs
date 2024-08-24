use core::char;

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

pub struct HashBridge<
    const NAME_HASH: u64,
    const FILE_NAME_HASH: u64,
    const COLUMN: u32,
    const LINE: u32,
>;

pub struct Def {
    str: &'static str,
}

// impl<const PTR: usize, const LEN: usize> ConstStructPrimQueue for StrPointerAndLength<PTR, LEN> {
//     type Data = &'static str;
//     const __DATA: Self::Data = unsafe {
//         core::str::from_utf8_unchecked(core::slice::from_raw_parts(PTR as *const u8, LEN))
//     };
// }

pub fn call_tester<T: ConstStructPrimQueue<Data = &'static str>>() -> &'static str {
    T::__DATA
}

macro_rules! TestSettingManual {
    ($value:expr) => {
        HashBridge<{
            const NAME_HASH: u64 = {
                let str = stringify!($value);
                let crc: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
                crc.checksum(str.as_bytes())
            };

            const FILE_HASH: u64 = {
                let file_name = file!();
                let crc: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
                crc.checksum(file_name.as_bytes())
            };

            impl ConstStructPrimQueue for HashBridge<NAME_HASH, FILE_HASH, {column!()}, {line!()}> {
                type Data = &'static str;
                const __DATA: Self::Data = $value;
            }

            NAME_HASH
        }, {
            const FILE_HASH: u64 = {
                let file_name = file!();
                let crc: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
                crc.checksum(file_name.as_bytes())
            };

            FILE_HASH
        }, {
            column!()
        }, {
            line!()
        }>
    }
}

#[test]
pub fn tester() {
    type B = TestSettingManual!("Hello, World!");

    let b = call_tester::<B>();

    // let a_struct = AStruct {
    //     str: "Hello, World!",
    // };

    assert_eq!(b, "Hello, World!");
}
