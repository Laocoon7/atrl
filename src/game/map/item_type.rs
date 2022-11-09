use crate::game::prelude::*;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(
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
#[repr(u8)]
pub enum ItemType {
    #[default]
    None,
}

impl From<ItemType> for u8 {
    fn from(value: ItemType) -> Self {
        ToPrimitive::to_u8(&value).expect("Failed to convert `TerrainType` to u8")
    }
}

impl From<ItemType> for u64 {
    fn from(value: ItemType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `ItemType` to u64")
    }
}
