use crate::{impl_raw, prelude::*};

#[derive(Deserialize, Debug, Clone)]
pub struct RawPlayer {
    pub name: String,
    pub stats: RawStats,
    pub vision_range: u8,
    pub vision: Vec<VisionType>,
    pub movement: Vec<MovementType>,
}

impl_raw!(RawPlayer);
