use crate::prelude::*;
#[derive(
    Reflect,
    FromReflect,
    Debug,
    Default,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[repr(u32)]
pub enum ItemType {
    #[default]
    None,
}

impl From<ItemType> for u32 {
    fn from(value: ItemType) -> Self {
        ToPrimitive::to_u32(&value).expect("Failed to convert `ItemType` to u32")
    }
}

impl From<ItemType> for u64 {
    fn from(value: ItemType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `ItemType` to u64")
    }
}
