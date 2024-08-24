use crate::pre::str_hash;
use crate::pre::PrimitiveTraits;

#[derive(Debug)]
pub struct TestStruct {
    test_data: [u8; 16],
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    str: &'static str,
}

pub trait TestStructTy: PrimitiveTraits<DATATYPE = TestStruct> {
    const TEST_DATA: [u8; 16] = <Self as PrimitiveTraits>::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = <Self as PrimitiveTraits>::__DATA.test_data2;
    const TEST_DATA3: u32 = <Self as PrimitiveTraits>::__DATA.test_data3;
    const STR: &'static str = <Self as PrimitiveTraits>::__DATA.str;
}

impl<U: PrimitiveTraits<DATATYPE = TestStruct>> TestStructTy for U {}

macro_rules! TestStruct {
    ($value:expr) => {
        HashBridge<{
            const NAME_HASH: u64 = str_hash(stringify!($value));

            impl PrimitiveTraits for HashBridge<NAME_HASH, {str_hash(file!())}, {column!()}, {line!()}> {
                type DATATYPE = TestStruct;
                const __DATA: Self::DATATYPE = $value;
            }

            NAME_HASH
        }, {
            str_hash(file!())
        }, {
            column!()
        }, {
            line!()
        }>
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pre::HashBridge;

    #[test]
    fn test() {
        type T = TestStruct!(TestStruct {
            test_data: [0; 16],
            test_data2: Some(Some(0)),
            test_data3: 0,
            str: "test",
        });

        let t: T = unsafe { core::mem::zeroed() };
        dbg!(t);

        assert_eq!(T::TEST_DATA, [0; 16]);
        assert_eq!(T::TEST_DATA2, Some(Some(0)));
        assert_eq!(T::TEST_DATA3, 0);
        assert_eq!(T::STR, "test");
    }
}
