pub trait PrimitiveTraits {
    type DATATYPE;
    const __DATA: Self::DATATYPE;
}

#[derive(Copy, Clone)]
pub struct HashBridge<
    const NAME_HASH: u64,
    const FILE_NAME_HASH: u64,
    const COLUMN: u32,
    const LINE: u32,
>;

// Debug
impl<const NAME_HASH: u64, const FILE_NAME_HASH: u64, const COLUMN: u32, const LINE: u32> core::fmt::Debug
    for HashBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashBridge")
            .field("NAME_HASH", &NAME_HASH)
            .field("FILE_NAME_HASH", &FILE_NAME_HASH)
            .field("COLUMN", &COLUMN)
            .field("LINE", &LINE)
            .finish()
    }
}
