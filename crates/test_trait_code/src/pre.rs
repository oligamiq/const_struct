pub trait PrimitiveTraits {
    type DATATYPE;
    const __DATA: Self::DATATYPE;
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
