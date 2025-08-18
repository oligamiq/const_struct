// #![no_std]

mod pre;
mod primitive;

mod generics;
mod hash_bridge;
mod keeptype;
mod non_generics;
mod with_const_generics;

fn main() {}

#[doc(hidden)]
#[allow(dead_code)]
pub(crate) struct ConstStructHashBridge<
    const NAME_HASH: u64,
    const FILE_NAME_HASH: u64,
    const COLUMN: u32,
    const LINE: u32,
>;
