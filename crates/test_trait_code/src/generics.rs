use crate::pre::{str_hash, PrimitiveTraits};

#[derive(Debug)]
pub struct TestStructWithGenerics<const T: usize> {
    test_data: Option<u32>,
    test_data2: Option<Option<u32>>,
    test_data3: u32,
    test_data4: [u8; T],
    str: &'static str,
}

pub trait TestStructWithGenericsTy<const T: usize>:
    PrimitiveTraits<DATATYPE = TestStructWithGenerics<{T}>>
{
    const TEST_DATA: Option<u32> = <Self as PrimitiveTraits>::__DATA.test_data;
    const TEST_DATA2: Option<Option<u32>> = <Self as PrimitiveTraits>::__DATA.test_data2;
    const TEST_DATA3: u32 = <Self as PrimitiveTraits>::__DATA.test_data3;
    const TEST_DATA4: [u8; T] = <Self as PrimitiveTraits>::__DATA.test_data4;
    const STR: &'static str = <Self as PrimitiveTraits>::__DATA.str;
}

impl<U: PrimitiveTraits<DATATYPE = TestStructWithGenerics<{T}>>, const T: usize> TestStructWithGenericsTy<{T}> for U {}

macro_rules! TestStructWithGenerics {
    ($value:expr) => {
        HashBridge<{
            const NAME_HASH: u64 = str_hash(stringify!($value));

            impl PrimitiveTraits for HashBridge<NAME_HASH, {str_hash(file!())}, {column!()}, {line!()}> {
                type DATATYPE = TestStructWithGenerics<{
                    const fn get_generic<const T: usize>(_: TestStructWithGenerics<{T}>) -> usize {
                        T
                    }

                    get_generic($value)
                }>;
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
        type T = TestStructWithGenerics!(TestStructWithGenerics {
            test_data: Some(1),
            test_data2: Some(Some(2)),
            test_data3: 3,
            test_data4: [0; 8],
            str: "test",
        });

        let t: T = unsafe { core::mem::zeroed() };
        dbg!(t);

        assert_eq!(T::TEST_DATA, Some(1));
        assert_eq!(T::TEST_DATA2, Some(Some(2)));
        assert_eq!(T::TEST_DATA3, 3);
        assert_eq!(T::TEST_DATA4, [0; 8]);
        assert_eq!(T::STR, "test");
    }
}
