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

impl<const NAME_HASH: u64, const FILE_NAME_HASH: u64, const COLUMN: u32, const LINE: u32> Default
    for HashBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE>
{
    fn default() -> Self {
        Self
    }
}

impl<const NAME_HASH: u64, const FILE_NAME_HASH: u64, const COLUMN: u32, const LINE: u32>
HashBridge<NAME_HASH, FILE_NAME_HASH, COLUMN, LINE> {
    pub const fn new() -> Self {
        Self
    }
}

pub const fn str_hash(s: &str) -> u64 {
    let crc: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
    crc.checksum(s.as_bytes())
}

#[macro_export]
macro_rules! match_underscore {
    (_, $tt_is_underscore:expr) => {
        $tt_is_underscore
    };
    ($input:expr, $tt_is_underscore:expr) => {
        $input
    };
}

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
