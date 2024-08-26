use crate::pre::PrimitiveTraits;

pub struct HashBridge<
    const NAME_HASH: u64,
    const FILE_NAME_HASH: u64,
    const COLUMN: u32,
    const LINE: u32,
    U,
> {
    __phantom: core::marker::PhantomData<U>,
}

impl<const NAME_HASH: u64, const FILE_NAME_HASH: u64, const COLUMN: u32, const LINE: u32, U> Default
    for HashBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE, U>
{
    fn default() -> Self {
        Self {
            __phantom: core::marker::PhantomData,
        }
    }
}

impl<const NAME_HASH: u64, const FILE_NAME_HASH: u64, const COLUMN: u32, const LINE: u32, U>
    HashBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE, U>
{
    pub const fn new() -> Self {
        Self {
            __phantom: core::marker::PhantomData,
        }
    }
}

// Debug
impl<const NAME_HASH: u64, const FILE_NAME_HASH: u64, const COLUMN: u32, const LINE: u32, U>
    core::fmt::Debug for HashBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE, U>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HashBridge")
            .field("NAME_HASH", &NAME_HASH)
            .field("FILE_NAME_HASH", &FILE_NAME_HASH)
            .field("COLUMN", &COLUMN)
            .field("LINE", &LINE)
            .field("U", &core::any::type_name::<U>())
            .finish()
    }
}

pub const fn str_hash(s: &str) -> u64 {
    let crc: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
    crc.checksum(s.as_bytes())
}

pub trait HashBridgeBridge<
    const NAME_HASH: u64,
    const FILE_NAME_HASH: u64,
    const COLUMN: u32,
    const LINE: u32,
>
{
    type DATATYPE;
    const DATA: Self::DATATYPE;
}

impl<
        U: HashBridgeBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE>,
        const NAME_HASH: u64,
        const FILE_NAME_HASH: u64,
        const COLUMN: u32,
        const LINE: u32,
    > PrimitiveTraits for HashBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE, U>
{
    type DATATYPE = U::DATATYPE;
    const __DATA: Self::DATATYPE = U::DATA;
}
