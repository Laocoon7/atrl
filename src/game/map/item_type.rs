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

impl From<ItemType> for u64 {
    fn from(value: ItemType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `ItemType` to u64")
    }
}

impl From<ItemType> for usize {
    fn from(value: ItemType) -> Self {
        match value {
            ItemType::None => 0,
        }
    }
}

impl From<ItemType> for Color {
    fn from(value: ItemType) -> Self {
        match value {
            ItemType::None => Color::NONE,
        }
    }
}
