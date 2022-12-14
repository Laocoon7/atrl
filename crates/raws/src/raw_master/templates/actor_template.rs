use crate::{impl_raw, prelude::*};

#[derive(Deserialize, Debug, Clone)]
pub struct RawActor {
    pub name: String,
    pub stats: RawStats,
    pub vision_range: u8,

    #[serde(default)]
    pub vision: Vec<VisionType>,

    #[serde(default)]
    pub movement: Vec<MovementType>,
}

impl_raw!(RawActor);
